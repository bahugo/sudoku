use std::cmp::min;
use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::palette::tailwind;
use ratatui::widgets::{BorderType, Borders, Padding, Wrap};
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, TableState},
    DefaultTerminal, Frame,
};
use unicode_width::UnicodeWidthStr;

use sudoku::{Board, BoardItem};

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub struct App {
    board: Board,
    state: TableState,
    colors: TableColors,
    longest_item_len: u16,
    rows_nb: usize,
    columns_nb: usize,
    exit: bool,
}

struct TableColors {
    buffer_bg: Color,
    cell_fg: Color,
    cell_bg: Color,
    selected_cell_style_fg: Color,
    defined_cell_bg: Color,
    defined_cell_style_fg: Color,
    error_cell_style_fg: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c900,
            cell_fg: tailwind::SLATE.c300,
            cell_bg: tailwind::SLATE.c900,
            defined_cell_bg: tailwind::SLATE.c950,
            selected_cell_style_fg: color.c600,
            defined_cell_style_fg: tailwind::INDIGO.c300,
            error_cell_style_fg: tailwind::RED.c300,
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    // let mut app = App::new(
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

// ANCHOR: impl App
impl App {
    fn new() -> Self {
        #[rustfmt::skip]
        let board = Board::new([
            [BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
            [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
            [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)],
        ]);

        Self {
            state: TableState::default().with_selected_cell((0, 0)),
            longest_item_len: App::constraint_len_calculator(&board),
            colors: TableColors::new(&PALETTES[2]),
            board,
            rows_nb: 9,
            columns_nb: 9,
            exit: false,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn constraint_len_calculator(board: &Board) -> u16 {
        board
            .array
            .iter()
            .flat_map(|x| x.iter().map(App::get_display_value_for_boarditem))
            .map(|v| UnicodeWidthStr::width(v.as_str()).try_into().unwrap_or(1))
            .max()
            .unwrap_or(1)
    }

    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Sudoku App ".bold());
        let instructions = Line::from(vec![
            " Move left".into(),
            "<Left or h>".blue().bold(),
            " Move Right".into(),
            "<Right or l>".blue().bold(),
            " Move Up".into(),
            "<Up or k>".blue().bold(),
            " Move Down".into(),
            "<Down or j>".blue().bold(),
            " delete item ".into(),
            "<x> ".red().bold(),
            " Solve ".into(),
            "<S> ".green().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .padding(Padding::new(0, 0, 0, 0))
            .border_set(border::THICK);

        frame.render_widget(block, frame.area());
        self.render_board(frame, frame.area());
    }

    fn render_board(&mut self, frame: &mut Frame, area: Rect) {
        const CELL_HEIGHT: u16 = 3;
        let cell_width: u16 = CELL_HEIGHT * 2;
        let row_constraints = vec![
            Constraint::Min(0),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(1),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(1),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Length(CELL_HEIGHT),
            Constraint::Min(0),
        ];
        let col_constraints = vec![
            Constraint::Min(0),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(2),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(2),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Min(0),
        ];

        let (row_rects, _row_spacers) = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .vertical_margin(0)
            .horizontal_margin(0)
            .split_with_spacers(area);

        for (r, row_rect) in row_rects
            .iter()
            .enumerate()
            // filter block spacers
            .filter_map(|(i, val)| {
                if i != 0 && i != 4 && i != 8 && i != 12 {
                    Some(val)
                } else {
                    None
                }
            })
            .enumerate()
        {
            let (col_rects , _col_spacers) = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(0)
                .horizontal_margin(0)
                .constraints(col_constraints.clone())
                .split_with_spacers(*row_rect);

            for (c, cell_rect) in col_rects
                .iter()
                .enumerate()
                // filter block spacers
                .filter_map(|(i, val)| {
                    if i != 0 && i != 4 && i != 8 && i != 12 {
                        Some(val)
                    } else {
                        None
                    }
                })
                .enumerate()
            {
                let single_row_text = self.get_display_value(r, c);
                let cell_bg = if self.is_undefined(r, c) {
                    self.colors.cell_bg
                } else {
                    self.colors.defined_cell_bg
                };
                let cell_fg = if self.is_selected(r, c) {
                    self.colors.selected_cell_style_fg
                } else if self.is_undefined(r, c) {
                    self.colors.defined_cell_style_fg
                } else if self.is_error(r, c) {
                    self.colors.error_cell_style_fg
                } else {
                    self.colors.cell_fg
                };
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(cell_bg).fg(cell_fg))
                    .padding(Padding::new(0, 0, 0, 0))
                    .border_type(BorderType::Plain);
                // cell background color
                let text_style = Style::default().bg(cell_bg);
                let cell_text = Paragraph::new(single_row_text)
                    .block(block)
                    .style(text_style)
                    .alignment(Alignment::Center)
                    .centered()
                    .wrap(Wrap{trim: true});
                frame.render_widget(cell_text, *cell_rect);
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // ANCHOR: handle_key_event fn
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => self.solve(),
            KeyCode::Char('h') => self.move_left(),
            KeyCode::Char('l') => self.move_right(),
            KeyCode::Char('k') => self.move_up(),
            KeyCode::Char('j') => self.move_down(),
            KeyCode::Char('x') => self.delete_item(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Char('1') => self.set_value_on_selected_cell(1),
            KeyCode::Char('2') => self.set_value_on_selected_cell(2),
            KeyCode::Char('3') => self.set_value_on_selected_cell(3),
            KeyCode::Char('4') => self.set_value_on_selected_cell(4),
            KeyCode::Char('5') => self.set_value_on_selected_cell(5),
            KeyCode::Char('6') => self.set_value_on_selected_cell(6),
            KeyCode::Char('7') => self.set_value_on_selected_cell(7),
            KeyCode::Char('8') => self.set_value_on_selected_cell(8),
            KeyCode::Char('9') => self.set_value_on_selected_cell(9),
            _ => {}
        }
    }
    // ANCHOR_END: handle_key_event fn

    fn solve(&mut self) {
        let board = self.board.solve().unwrap();
        self.board = board;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_left(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        if selected_cell.1 == 0 {
            return;
        }
        let selected_col = selected_cell.1 - 1;
        self.state
            .select_cell(Some((selected_cell.0, selected_col)));
    }

    fn move_right(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        let selected_col = min(selected_cell.1 + 1, self.columns_nb - 1);
        self.state
            .select_cell(Some((selected_cell.0, selected_col)));
    }

    fn move_up(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        if selected_cell.0 == 0 {
            return;
        }
        let selected_row = selected_cell.0 - 1;
        self.state
            .select_cell(Some((selected_row, selected_cell.1)));
    }
    fn move_down(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        let selected_row = min(selected_cell.0 + 1, self.rows_nb - 1);
        self.state
            .select_cell(Some((selected_row, selected_cell.1)));
    }

    fn delete_item(&mut self) {
        if let Some((selected_row, selected_col)) = self.state.selected_cell() {
            let mut array = self.board.array.clone();
            array[selected_row][selected_col] = BoardItem::unknown();
            self.board = Board::new(array);
        };
    }

    fn set_value_on_selected_cell(&mut self, value: u8) {
        if let Some((selected_row, selected_col)) = self.state.selected_cell() {
            self.board.array[selected_row][selected_col].value = Some(value);
        };
    }

    fn get_display_value_for_boarditem(item: &BoardItem) -> String {
        item.value.map(|x| x.to_string()).unwrap_or(" ".to_string())
    }

    fn get_display_value(&self, row: usize, col: usize) -> String {
        App::get_display_value_for_boarditem(&self.board.array[row][col])
    }

    fn is_undefined(&self, row: usize, col: usize) -> bool {
        self.board.array[row][col].value.is_none()
    }

    fn is_error(&self, row: usize, col: usize) -> bool {
        !self.board.is_cell_valid(row,col)
    }

    fn is_selected(&self, row: usize, col: usize) -> bool {
        row == self.state.selected().unwrap_or(0)
            && col == self.state.selected_column().unwrap_or(0)
    }
}
// ANCHOR_END: impl App
