/*
   Object oriented features
*/


pub struct AveragdeCollection {
    list: Vec<i32>,
    average: f64,
}


impl AveragdeCollection {
    fn add(&self, number: i32) {
        self.list.push(number);
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let sum: f64 = self.list.iter().sum();
        self.average = sum/self.list.len() as f64;
    }
}

fn main() {




}
