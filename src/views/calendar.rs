use chrono::Datelike;

fn days_in_month(year: i32, month: u32) -> u32 {
    chrono::NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}

pub fn show_calendar(ui: &mut egui::Ui) {
    ui.set_width(350.0);
    ui.add_space(10.0);

    let now = chrono::Local::now();

    // Month and Year header
    ui.vertical_centered(|ui| {
        ui.heading(now.format("%B %Y").to_string());
    });
    ui.add_space(5.0);

    // Day headers
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
        for day in ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] {
            ui.label(egui::RichText::new(day).strong().size(12.0));
            ui.add_space(15.0);
        }
    });

    ui.add_space(5.0);

    // Calendar grid
    let first_day = now.with_day(1).unwrap();
    let first_weekday = first_day.weekday().num_days_from_sunday() as usize;
    let days_in_month = days_in_month(now.year(), now.month()) as usize;
    let today = now.day() as usize;

    let mut day = 1;
    let total_cells = ((first_weekday + days_in_month + 6) / 7) * 7;

    for week in 0..(total_cells / 7) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
            for weekday in 0..7 {
                let cell_index = week * 7 + weekday;
                if cell_index < first_weekday || day > days_in_month {
                    ui.add_space(35.0);
                } else {
                    let is_today = day == today;
                    let day_text = if is_today {
                        egui::RichText::new(format!("{}", day))
                            .strong()
                            .color(egui::Color32::from_rgb(0, 120, 215))
                    } else {
                        egui::RichText::new(format!("{}", day))
                    };
                    ui.label(day_text);
                    ui.add_space(if day < 10 { 23.0 } else { 17.0 });
                    day += 1;
                }
            }
        });
    }

    ui.add_space(10.0);
}
