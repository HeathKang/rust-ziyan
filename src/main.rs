use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::LinkedList;

fn main() {
    let mut check_vector: VecDeque<QueueData> = VecDeque::new();
    let queuedata: QueueData = generate();

    check_vector.push_back(queuedata);

    println!("{:?}", check_vector.pop_back().unwrap());
}

fn generate() -> QueueData {
    let data = Data::Stri(String::from("now"));
    let status = Data::Int32(1);
    let mut list: LinkedList<Data> = LinkedList::new();

    list.push_back(data);
    list.push_back(status);

    return QueueData::DataList(list)
}

#[derive(Debug)]
enum QueueData {
   DataList(LinkedList<Data>),
   DataMap(HashMap<String, Data>), 
}

#[derive(Debug)]
enum Data {
    Stri(String),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),

}
