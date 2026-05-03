// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Ratatui dashboard application loop.

use std::{
    fmt, io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline, Wrap},
};

use super::{
    DashboardRuntimeContext,
    action::{DashboardAction, DashboardActionState, action_confirm_text, confirm_and_execute},
    collect_dashboard_snapshot,
    model::DashboardState,
};
use crate::operator::DashboardArgs;

const MIN_INTERACTIVE_DASHBOARD_HEIGHT: u16 = 25;

/// Interactive dashboard error.
#[derive(Debug)]
pub struct DashboardAppError {
    message: String,
}

impl DashboardAppError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for DashboardAppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DashboardAppError {}

impl From<io::Error> for DashboardAppError {
    fn from(error: io::Error) -> Self {
        Self::new(error.to_string())
    }
}

/// Run the live terminal dashboard until the operator exits.
pub fn run_interactive_dashboard(
    args: &DashboardArgs,
    context: &DashboardRuntimeContext,
) -> Result<(), DashboardAppError> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_loop(args, context, &mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run_loop(
    args: &DashboardArgs,
    context: &DashboardRuntimeContext,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), DashboardAppError> {
    let tick_rate = Duration::from_millis(args.tick_ms.max(250));
    let mut last_tick = Instant::now();
    let mut state = DashboardState::from_snapshot(&collect_dashboard_snapshot(context));
    let mut maybe_pending_action: Option<DashboardAction> = None;
    let mut message =
        String::from("r refresh | s service status | i/u/e/d service action | q quit");

    loop {
        let mut dashboard_blocked = false;
        terminal.draw(|frame| {
            dashboard_blocked = !is_interactive_dashboard_height_sufficient(frame.area().height);
            draw_dashboard(
                frame,
                &state,
                maybe_pending_action,
                &message,
                dashboard_blocked,
            );
        })?;
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            let action = action_for_key(key);
            if handle_dashboard_action(
                action,
                dashboard_blocked,
                context,
                &mut state,
                &mut maybe_pending_action,
                &mut message,
            )? {
                break;
            }
        }
        if last_tick.elapsed() >= tick_rate {
            state = DashboardState::from_snapshot(&collect_dashboard_snapshot(context));
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn draw_dashboard(
    frame: &mut Frame<'_>,
    state: &DashboardState,
    maybe_pending_action: Option<DashboardAction>,
    message: &str,
    dashboard_blocked: bool,
) {
    let area = frame.area();
    if dashboard_blocked {
        render_small_window_blocker(frame, area);
        return;
    }

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(8),
            Constraint::Length(4),
        ])
        .split(area);

    let title = Paragraph::new("Open Bitcoin Dashboard")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title("Operator"));
    frame.render_widget(title, outer[0]);

    let section_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Percentage(30),
        ])
        .split(outer[1]);
    let left = render_sections(&state.sections[..2]);
    let middle = render_sections(&state.sections[2..4]);
    let right = render_sections(&state.sections[4..]);
    frame.render_widget(left, section_area[0]);
    frame.render_widget(middle, section_area[1]);
    frame.render_widget(right, section_area[2]);

    let chart_constraints = state
        .charts
        .iter()
        .map(|_| Constraint::Ratio(1, state.charts.len() as u32))
        .collect::<Vec<_>>();
    let chart_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(chart_constraints)
        .split(outer[2]);
    for (index, chart) in state.charts.iter().enumerate() {
        let sparkline = Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} ({})", chart.title, chart.availability)),
            )
            .style(Style::default().fg(Color::Green))
            .data(&chart.points);
        frame.render_widget(sparkline, chart_areas[index]);
    }

    let prompt = if let Some(action) = maybe_pending_action {
        action_confirm_text(action, true)
    } else {
        message.to_string()
    };
    let action_line = render_action_line(&state.actions);
    let actions = Paragraph::new(vec![Line::from(action_line), Line::from(prompt)])
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Actions"));
    frame.render_widget(actions, outer[3]);
}

fn is_interactive_dashboard_height_sufficient(height: u16) -> bool {
    height >= MIN_INTERACTIVE_DASHBOARD_HEIGHT
}

fn render_small_window_blocker(frame: &mut Frame<'_>, area: Rect) {
    let body = vec![
        Line::from(Span::styled(
            "Interactive dashboard unavailable",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!(
            "This dashboard needs at least {MIN_INTERACTIVE_DASHBOARD_HEIGHT} rows to show confirmation and help text safely."
        )),
        Line::from(format!(
            "Current height: {} rows. Required height: {MIN_INTERACTIVE_DASHBOARD_HEIGHT} rows.",
            area.height
        )),
        Line::from(""),
        Line::from("Resize the terminal to continue. Press q, Esc, or Ctrl-C to quit."),
    ];
    let blocker = Paragraph::new(body).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Window too small"),
    );
    frame.render_widget(blocker, area);
}

fn render_action_line(actions: &[super::model::ActionEntry]) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    for (index, action) in actions.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
        }

        let style = if action.destructive {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        spans.push(Span::styled(
            format!("{} {}", action.key, action.label),
            style,
        ));
    }

    spans
}

fn render_sections(sections: &[super::model::DashboardSection]) -> List<'_> {
    let items = sections
        .iter()
        .flat_map(|section| {
            let mut items = vec![ListItem::new(Line::from(Span::styled(
                section.title.clone(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )))];
            items.extend(section.rows.iter().map(|row| {
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{}: ", row.label), Style::default().fg(Color::Gray)),
                    Span::raw(row.value.clone()),
                ]))
            }));
            items
        })
        .collect::<Vec<_>>();

    List::new(items).block(Block::default().borders(Borders::ALL))
}

fn action_for_key(key: KeyEvent) -> DashboardAction {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            DashboardAction::Exit
        }
        KeyCode::Esc | KeyCode::Char('q') => DashboardAction::Exit,
        KeyCode::Char('r') => DashboardAction::Refresh,
        KeyCode::Char('s') => DashboardAction::ShowStatus,
        KeyCode::Char('i') => DashboardAction::InstallService,
        KeyCode::Char('u') => DashboardAction::UninstallService,
        KeyCode::Char('e') => DashboardAction::EnableService,
        KeyCode::Char('d') => DashboardAction::DisableService,
        KeyCode::Char('h') | KeyCode::Char('?') => DashboardAction::Help,
        KeyCode::Char('y') => DashboardAction::Confirm,
        KeyCode::Char('n') => DashboardAction::Cancel,
        _ => DashboardAction::None,
    }
}

fn handle_dashboard_action(
    action: DashboardAction,
    dashboard_blocked: bool,
    context: &DashboardRuntimeContext,
    state: &mut DashboardState,
    maybe_pending_action: &mut Option<DashboardAction>,
    message: &mut String,
) -> Result<bool, DashboardAppError> {
    if dashboard_blocked {
        return Ok(matches!(action, DashboardAction::Exit));
    }

    handle_action(action, context, state, maybe_pending_action, message)
}

fn handle_action(
    action: DashboardAction,
    context: &DashboardRuntimeContext,
    state: &mut DashboardState,
    maybe_pending_action: &mut Option<DashboardAction>,
    message: &mut String,
) -> Result<bool, DashboardAppError> {
    if let Some(pending_action) = maybe_pending_action.take() {
        match action {
            DashboardAction::Cancel | DashboardAction::None => {
                *message = "confirmation cancelled".to_string();
                return Ok(false);
            }
            DashboardAction::Exit => return Ok(true),
            DashboardAction::Confirm => {
                let outcome = confirm_and_execute(
                    &DashboardActionState::confirmed(pending_action),
                    &context.service.as_execution_context(),
                );
                *message = if outcome.exit_code.code() == 0 {
                    outcome.stdout.text
                } else {
                    outcome.stderr.text
                };
                return Ok(false);
            }
            _ => {
                *maybe_pending_action = Some(pending_action);
                *message = action_confirm_text(pending_action, true);
                return Ok(false);
            }
        }
    }

    match action {
        DashboardAction::Exit => Ok(true),
        DashboardAction::Refresh => {
            *state = DashboardState::from_snapshot(&collect_dashboard_snapshot(context));
            *message = "refreshed".to_string();
            Ok(false)
        }
        DashboardAction::ShowStatus => {
            let outcome = confirm_and_execute(
                &DashboardActionState::confirmed(DashboardAction::ShowStatus),
                &context.service.as_execution_context(),
            );
            *message = if outcome.exit_code.code() == 0 {
                outcome.stdout.text
            } else {
                outcome.stderr.text
            };
            Ok(false)
        }
        DashboardAction::InstallService
        | DashboardAction::UninstallService
        | DashboardAction::EnableService
        | DashboardAction::DisableService => {
            *maybe_pending_action = Some(action);
            *message = action_confirm_text(action, true);
            Ok(false)
        }
        DashboardAction::Help => {
            *message = "r refresh, s status, i install, u uninstall, e enable, d disable, q quit"
                .to_string();
            Ok(false)
        }
        DashboardAction::Confirm | DashboardAction::Cancel => Ok(false),
        DashboardAction::None => Ok(false),
    }
}

#[cfg(test)]
mod tests;
