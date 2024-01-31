fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        min-width: 200px;
        min-height: 100px;
        padding: 10px;        
        background: #545454;
        Text {
            text: "hello world";
            color: whitesmoke;
        }
    }
}

