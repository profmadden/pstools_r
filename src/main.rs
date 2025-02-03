use pstools_r::{*};

fn main() {
    println!("PSTools main!");
    let mut pst = PSTool::new();

    pst.set_fill(true);
    pst.set_color(0.3, 0.4, 0.2, 1.0);
    pst.add_box(5.0, 5.0, 20.0, 30.0);
    pst.set_color(0.8, 0.1, 0.2, 1.0);
    pst.add_box(35.0, 19.0, 7.0, 16.0);
    pst.add_line(33.2, 44.1, 8.7, 5.5);
    
    pst.set_color(0.1, 0.1, 0.8, 1.0);
    pst.set_fill(false);
    pst.add_box(50.0, 50.0, 60.0, 60.0);

    pst.set_color(0.1, 0.1, 0.8, 1.0);
    pst.set_fill(true);
    pst.add_box(70.0, 70.0, 80.0, 80.0);

    pst.set_color(0.0, 0.0, 0.0, 1.0);
    pst.add_text(10.0, 10.0, "1Hello World".to_string());
    
    pst.set_color(0.0, 0.0, 1.0, 1.0);
    pst.set_font(20.0, "Times-Roman".to_string());
    pst.add_text(10.0, 30.0, "2Hello World".to_string());

    pst.set_font(20.0, "Helvetica".to_string());
    pst.set_color(0.0, 1.0, 0.0, 1.0);
    pst.add_text(10.0, 50.0, "3Hello World".to_string());
    
    pst.set_border(10.0);

    pst.add_circle(20.0, 20.0, 8.0);
    pst.set_color(1.0, 0.0, 0.0, 1.0);
    pst.set_fill(false);
    pst.add_circle(22.0, 22.0, 9.0);

    pst.generate("testfile.ps".to_string());

    // pst.generate("testfile2.ps".to_string());
    // pst.set_bounds(8.0, 3.0, 80.0, 40.0);
    // pst.generate("testfile3.ps".to_string());

}