use web_view::*;

fn main() {
    struct UserData {
        count: u32,
    }

    let webview = web_view::builder()
        .title("Cube training")
        .content(Content::Html(HTML))
        .size(640, 480)
        .resizable(false)
        .debug(true)
        .user_data( UserData { count: 0 } )
        .invoke_handler(|_webview, arg| {
            println!("xCount!: {}",arg);
            match arg {
                "countup" => {
                    _webview.user_data_mut().count += 1;
                    eprintln!("Count!");
                    _webview.eval(&format!("update({})", _webview.user_data().count))
                }
                _ => Ok(()),
            }
        })
        .build()
        .unwrap();
    
    webview.run().unwrap();
}

const HTML: &str = r#"<!DOCTYPE html>
<html>
    <body>
        <div id="test">start</div>
        <div id="display">0</div>
        <button onclick="test()">count</button>
        <script>
            function update(count) {
                document.getElementById('display').innerHTML = count;
            }
            function test() {
                check = false;
                document.getElementById('test').innerHTML = 'a';
                if (external == undefined){
                    document.getElementById('test').innerHTML = 'b';
                    check = true;
                };
                document.getElementById('test').innerHTML = 'c';
                alert(check);
                external.invoke('countup');
            }
        </script>
    </body>
</html>
"#;