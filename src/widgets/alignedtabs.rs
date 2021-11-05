use tui::{
    buffer::Buffer,
    layout::{Rect, Alignment},
    style::Style,
    symbols,
    text::{Span, Spans},
    widgets::{Block, Widget},
};

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Tabs};
/// # use tui::style::{Style, Color};
/// # use tui::text::{Spans};
/// # use tui::symbols::{DOT};
/// let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(Spans::from).collect();
/// Tabs::new(titles)
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow))
///     .divider(DOT);
/// ```
#[derive(Debug, Clone)]
pub struct AlignedTabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<Spans<'a>>,
    /// The index of the selected tabs
    selected: usize,
    /// The style used to draw the text
    style: Style,
    /// Style to apply to the selected item
    highlight_style: Style,
    /// Tab divider
    divider: Span<'a>,
    /// Alignment
    alignment: Alignment,
}

#[allow(dead_code)]
impl<'a> AlignedTabs<'a> {
    pub fn new(titles: Vec<Spans<'a>>) -> AlignedTabs<'a> {
        AlignedTabs {
            block: None,
            titles,
            selected: 0,
            style: Default::default(),
            highlight_style: Default::default(),
            divider: Span::raw(symbols::line::VERTICAL),
            alignment: Alignment::Left,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> AlignedTabs<'a> {
        self.block = Some(block);
        self
    }

    pub fn select(mut self, selected: usize) -> AlignedTabs<'a> {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> AlignedTabs<'a> {
        self.style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> AlignedTabs<'a> {
        self.highlight_style = style;
        self
    }

    pub fn divider<T>(mut self, divider: T) -> AlignedTabs<'a>
    where
        T: Into<Span<'a>>,
    {
        self.divider = divider.into();
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> AlignedTabs<'a>{
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for AlignedTabs<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if tabs_area.height < 1 {
            return;
        }

        // this is a sketchy way of implementing alignment
        // all it does is calculate the total width of the rendered tabs and set the offset accordingly
        // when using an alignment that is not Left:
        // does no checks to see if gone out of bounds or prevent rendering in such a case
        //
        // ---my code---
        let mut total_width = 0;
        for title in self.titles.iter() {
            total_width += title.width();
        }

        total_width += (self.divider.width() + 2) * self.titles.len() - 1;

        let offset = get_line_offset(total_width as u16, tabs_area.width, self.alignment);

        let mut x = tabs_area.left().saturating_add(offset);
        // ---my code---

        // let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        for (i, title) in self.titles.into_iter().enumerate() {
            let last_title = titles_length - 1 == i;
            x = x.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }
            let pos = buf.set_spans(x, tabs_area.top(), &title, remaining_width);
            if i == self.selected {
                buf.set_style(
                    Rect {
                        x,
                        y: tabs_area.top(),
                        width: pos.0.saturating_sub(x),
                        height: 1,
                    },
                    self.highlight_style,
                );
            }
            x = pos.0.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 || last_title {
                break;
            }
            let pos = buf.set_span(x, tabs_area.top(), &self.divider, remaining_width);
            x = pos.0;
        }
    }
}

fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Center => (text_area_width / 2).saturating_sub(line_width / 2),
        Alignment::Right => text_area_width.saturating_sub(line_width),
        Alignment::Left => 0
    }
}
