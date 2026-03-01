use crate::skills::Skill;
use crossterm::event::{self, Event, KeyCode};
use std::collections::HashMap;
use std::io::{self, Write};

#[allow(dead_code)]
pub struct Picker;

#[allow(dead_code)]
impl Picker {
    pub fn select_skills(
        skills: &HashMap<String, Skill>,
        already_selected: &[String],
    ) -> Vec<String> {
        let mut selected: Vec<String> = already_selected.to_vec();
        let mut cursor_pos = 0;
        let mut filter = String::new();
        let skill_names: Vec<String> = skills.keys().cloned().collect();

        loop {
            let display_names: Vec<String> = if filter.is_empty() {
                skill_names.clone()
            } else {
                skill_names
                    .iter()
                    .filter(|n| n.to_lowercase().contains(&filter.to_lowercase()))
                    .cloned()
                    .collect()
            };

            if cursor_pos >= display_names.len() {
                cursor_pos = display_names.len().saturating_sub(1);
            }

            print!("\x1B[2J\x1B[1H");
            println!("Select skills ( Esc to confirm, Space to toggle, Backspace to filter):");
            println!("{}", "─".repeat(50));

            for (i, name) in display_names.iter().enumerate() {
                let is_selected = selected.contains(name);
                let marker = if is_selected { "[*]" } else { "[ ]" };
                let cursor_marker = if i == cursor_pos { ">" } else { " " };

                if i == cursor_pos {
                    print!("\x1B[7m");
                }
                println!("{} {} {}", cursor_marker, marker, name);
                if i == cursor_pos {
                    print!("\x1B[0m");
                }
            }

            println!("\n{}", "─".repeat(50));
            println!("Filter: {}", filter);
            io::stdout().flush().ok();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        cursor_pos = cursor_pos.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        cursor_pos = (cursor_pos + 1).min(display_names.len().saturating_sub(1));
                    }
                    KeyCode::Enter => {
                        if let Some(name) = display_names.get(cursor_pos) {
                            if selected.contains(name) {
                                selected.retain(|n| n != name);
                            } else {
                                selected.push(name.clone());
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        filter.push(c);
                        cursor_pos = 0;
                    }
                    KeyCode::Backspace => {
                        filter.pop();
                        cursor_pos = 0;
                    }
                    _ => {}
                }
            }
        }

        selected
    }
}
