use ratatui::{
    style::{Color, Styled},
    widgets::Block,
};

pub fn set_focused_block(block: &mut Block, focused: bool) {
    let block_color = if focused {
        Color::LightBlue
    } else {
        Color::White
    };
    *block = Block::style(block.clone(), Styled::style(block).fg(block_color));
}
