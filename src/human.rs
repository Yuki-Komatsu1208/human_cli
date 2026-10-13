use chrono::{Datelike, Local, NaiveDate};

struct Human {
    name: String,
    born_at: NaiveDate,
    eated_foods: Vec<String>,
}

impl Human {
    fn new(name: String, born_at: NaiveDate) -> Self {
        Self {
            name,
            born_at,
            eated_foods: Vec::new(),
        }
    }

    pub fn eat(&mut self, food: String) {
        self.eated_foods.push(food);
    }

    pub fn eaten_foods(&self) -> &[String] {
        &self.eated_foods
    }
    ///年齢を取得
    pub fn age(&self) -> i32 {
        let today = Local::now().date_naive();
        let mut age = today.year() - self.born_at.year();
        if (today.month(), today.day()) < (self.born_at.month(), self.born_at.day()) {
            age -= 1;
        }
        age
    }
}
