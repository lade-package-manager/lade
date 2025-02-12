// test of install script
// to run this script. please execute `cargo run -- rhai ./tets.rhai`

let file = open_file("./test_rhai");

file.write("Hello write!");
file.write("I can add file content");

set_env("RHAI_TEST", "test of rhai");
let env = get_env("RHAI_TEST");

let clear = get_env("RHAI_CLEAR");
if clear == "yes"{
    print("File content is removed by rhai");
    file.clear();
}

let path = path("./test_rhai");
print(path.to_string());

if path.exists(){
    let msg = path.to_string() + " is exists!";
    print(msg);
}

print(env);