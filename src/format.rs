use console::{style, StyledObject};
use tabled::{Table, Tabled};

pub fn table<T>(iter: impl IntoIterator<Item = T>)
where
    T: Tabled,
{
    let mut table = Table::new(iter).with(tabled::Style::rounded());
    if let Some((w, _)) = term_size::dimensions() {
        table = table.with(tabled::Width::wrap(w));
    }
    println!("{}", table);
}

pub fn link(msg: impl ToString) -> StyledObject<String> {
    style(msg.to_string()).italic().underlined()
}
