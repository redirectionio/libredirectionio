//#[macro_use]
//extern crate lazy_static;
//mod router;
//
//#[no_mangle]
//pub extern "C" fn new_router() {
//    println!("Create a new router with a set of rules");
//}
//
//#[no_mangle]
//pub extern "C" fn router_match() {
//    println!("return a matching rule or null");
//}
//
//#[no_mangle]
//pub extern "C" fn router_filter_headers() {
//    println!("filter some headers with given headers and a rule");
//}
//
//#[no_mangle]
//pub extern "C" fn router_create_filter_body() {
//    println!("get a pointer to a body filter resource for a specific rule, or null if no filter for this body");
//}
//
//#[no_mangle]
//pub extern "C" fn router_filter_body_update() {
//    println!("update body filter with new data and return data filter");
//}
//
//#[no_mangle]
//pub extern "C" fn router_filter_body_close() {
//    println!("stop body filtering with optionally some last data and return optionally return missing data");
//}
