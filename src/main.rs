use std::cmp::min;
use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::palette::tailwind;
use ratatui::widgets::{BorderType, Borders};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{self, Color, Modifier, Style, Stylize},
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
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
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

fn constraint_len_calculator(board: &Board) -> u16 {
    board
        .array
        .iter()
        .flat_map(|x| x.iter().map(|y| format!("{}", y)))
        .map(|v| UnicodeWidthStr::width(v.as_str()).try_into().unwrap_or(1))
        .max()
        .unwrap_or(1)
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
            longest_item_len: constraint_len_calculator(&board),
            colors: TableColors::new(&PALETTES[0]),
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

    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Sudoku App ".bold());
        let instructions = Line::from(vec![
            " Move left (h)".into(),
            "<Left>".blue().bold(),
            " Move Right (l)".into(),
            "<Right>".blue().bold(),
            " Move Up (k)".into(),
            "<Up>".blue().bold(),
            " Move Down (j)".into(),
            "<Down>".blue().bold(),
            " Solve ".into(),
            "<S> ".green().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        frame.render_widget(block, frame.area());
        self.render_board(frame, frame.area());
    }

    fn render_board(&mut self, frame: &mut Frame, area: Rect) {
        const CELL_HEIGHT: u16 = 3;
        let CELL_WIDTH: u16 = self.longest_item_len;
        let rows_count = self.board.array.len();
        let cols_count = self.board.array[0].len();
        let row_constraints = std::iter::repeat(Constraint::Length(CELL_HEIGHT))
            .take(rows_count)
            .collect::<Vec<_>>();
        let col_constraints = std::iter::repeat(Constraint::Length(CELL_WIDTH))
            .take(cols_count)
            .collect::<Vec<_>>();

        let row_rects = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .horizontal_margin(0)
            .constraints(row_constraints)
            .split(area);

        for (r, row_rect) in row_rects.iter().enumerate() {
            let col_rects = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(0)
                .horizontal_margin(1)
                .constraints(col_constraints.clone())
                .split(*row_rect);

            for (c, cell_rect) in col_rects.iter().enumerate() {
                let item = &self.board.array[r][c];

                let single_row_text =
                    format!("{:^length$}", item, length = usize::from(CELL_WIDTH - 1));
                let pad_line = " ".repeat(usize::from(CELL_WIDTH));

                // 1 line for the text, 1 line each for the top and bottom of the cell == 3 lines
                // that are not eligible for padding
                let num_pad_lines = usize::from(CELL_HEIGHT.checked_sub(3).unwrap_or_default());

                // text is:
                //   pad with half the pad lines budget
                //   the interesting text
                //   pad with half the pad lines budget
                //   join with newlines
                let text = std::iter::repeat(pad_line.clone())
                    .take(num_pad_lines / 2)
                    .chain(std::iter::once(single_row_text.clone()))
                    .chain(std::iter::repeat(pad_line).take(num_pad_lines / 2))
                    .collect::<Vec<_>>()
                    .join("\n");
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black).fg(
                        // cell  border color
                        if self.is_selected(r, c) {
                            self.colors.selected_cell_style_fg
                        } else if self.is_active(r, c) {
                            todo!()
                        } else {
                            Color::White
                        },
                    ))
                    .border_type(BorderType::Rounded);
                // cell background color
                let text_style = Style::default().bg(Color::Black);
                let cell_text = Paragraph::new(text).block(block).style(text_style);
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
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            _ => {}
        }
    }
    // ANCHOR_END: handle_key_event fn

    fn solve(&mut self) {
        let board = self.board.solve_naive_implementation().unwrap();
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
        self.state.select_cell(Some((selected_cell.0, selected_col)));
    }

    fn move_right(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        let selected_col = min(selected_cell.1 + 1, self.columns_nb - 1);
        self.state.select_cell(Some((selected_cell.0, selected_col)));
    }

    fn move_up(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        if selected_cell.0 == 0 {
            return;
        }
        let selected_row = selected_cell.0 - 1;
        self.state.select_cell(Some((selected_row, selected_cell.1)));
    }
    fn move_down(&mut self) {
        let selected_cell = self.state.selected_cell().unwrap_or((0, 0));
        let selected_row = min(selected_cell.0 + 1, self.rows_nb - 1);
        self.state.select_cell(Some((selected_row, selected_cell.1)));
    }

    fn is_active(&self, row: usize, col: usize) -> bool {
        //FIXME
        false
    }

    fn is_selected(&self, row: usize, col: usize) -> bool {
        row == self.state.selected().unwrap_or(0)
            && col == self.state.selected_column().unwrap_or(0)
    }
}
// ANCHOR_END: impl App
