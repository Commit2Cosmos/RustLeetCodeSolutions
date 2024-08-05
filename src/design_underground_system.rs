use std::collections::HashMap;

struct UndergroundSystem {
    //* id -> (station_name, check_in_time)
    check_in_map: HashMap<i32, (String, i32)>,

    //* (check_in_station_name, check_out_station_name) -> [total_time, number_of_trips]
    time_map: HashMap<(String, String), (i32, usize)>
}


impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem { 
            check_in_map: HashMap::new(),
            time_map: HashMap::new()
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in_map.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (check_in_station_name, check_in_time) = self.check_in_map.get(&id).unwrap();
        let time_diff = t-check_in_time;
        self.time_map.entry((check_in_station_name.to_string(), station_name)).and_modify(|x| *x = (x.0+time_diff, (x.1+1))).or_insert((time_diff, 1));
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let &(total_time, number_of_trips) = self.time_map.get(&(start_station, end_station)).unwrap();
        total_time as f64 / number_of_trips as f64
    }
}


fn main() {
    let mut obj = UndergroundSystem::new();
    let id = 1;
    let start_station = String::from("Wat");
    let end_station = String::from("Re");

    obj.check_in(id, start_station.clone(), 10);
    obj.check_out(id, end_station.clone(), 12);
    obj.check_in(id, start_station.clone(), 24);
    obj.check_out(id, end_station.clone(), 30);

    let ret_3: f64 = obj.get_average_time(start_station, end_station);
    println!("{:?}", ret_3);
}