mod text;
mod map;

fn main() { 
    text::display::primary(text::wording::get().home_screen.title.to_string());
    text::display::success("█".to_string());
    map::mapLoader::load("sample/map/test.png");
}
