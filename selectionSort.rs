fn find_smallest(items:  &Vec<i32>) -> usize {
  let mut smallest = items[0];
  let mut smallest_index:usize = 0;
  for (i,number) in items.iter().enumerate() {
    if smallest >= *number {
      smallest = *number;
      smallest_index=i
    }
  }
  return smallest_index;
}
fn selection_sort(items: &mut Vec<i32>) -> Vec<i32> {
  let mut sorted_array :Vec<i32> = Vec::new();
  for _i in 0..items.len() {
    let smallest = find_smallest(&items);
    sorted_array.push(items.remove(smallest));
  }
  return sorted_array;
}
fn main(){
  let mut items = vec![9,10,15,22,43,57,5, 4, 3, 2, 1,8000,500,400,100,0];
  let result = selection_sort(&mut items);
  println!("{:?}",result);
}