use std::io;

fn displayIntro() -> u32 {
    let mut b1 = String::new();

    println!("Hello, Welcome to the grade calculator.  Please choose an option - ");
    println!("Calculate a new class grade - 1 ");
    println!("Calculate what test scores you need - 2 ");
    println!("Calculate what homework scores you need - 3 ");
    println!("End program - 4 ");

    loop {
        io::stdin().read_line(&mut b1).expect("Failed to read line");

        let b1: u32 = match b1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return b1;
    }
}

fn testScoreWeight() -> u32 {
    println!("What percent of your final grade are your test scores? ");

    let mut t = String::new();

    loop {
        io::stdin().read_line(&mut t).expect("Failed to read line");

        let t: u32 = match t.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return t;
    }
}

fn projectWeight() -> u32 {
    println!("What percent of your final grade are your projects? ");

    let mut p = String::new();

    loop {
        io::stdin().read_line(&mut p).expect("Failed to read line");

        let p: u32 = match p.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return p;
    }
}

fn homeworkWeight() -> u32 {
    println!("What percent of your final grade is your homework? ");

    let mut h = String::new();
    loop {
        io::stdin().read_line(&mut h).expect("Failed to read line");

        let h: u32 = match h.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return h;
    }
}

fn otherWeight() -> u32 {
    println!("What percent of your final grade is anything else? ");

    let mut o = String::new();
    loop {
        io::stdin().read_line(&mut o).expect("Failed to read line");

        let o: u32 = match o.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return o;
    }
}

fn testScoreAverage() -> u32 {
    println!("What average score did you recieve for your test scores? ");

    let mut tt = String::new();
    loop {
        io::stdin().read_line(&mut tt).expect("Failed to read line");

        let tt: u32 = match tt.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return tt;
    }
}

fn projectAverage() -> u32 {
    println!("What average score did you recieve for your projects? ");

    let mut pp = String::new();
    loop {
        io::stdin().read_line(&mut pp).expect("Failed to read line");

        let pp: u32 = match pp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return pp;
    }
}

fn homeworkAverage() -> u32 {
    println!("What average score did you recieve for your homework? ");
    let mut hh = String::new();

    loop {
        io::stdin().read_line(&mut hh).expect("Failed to read line");

        let hh: u32 = match hh.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return hh;
    }
}

fn otherAverage() -> u32 {
    println!("What average score did you recieve for anything else? ");
    let mut oo = String::new();

    loop {
        io::stdin().read_line(&mut oo).expect("Failed to read line");

        let oo: u32 = match oo.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return oo;
    }
}

fn finalTest() -> u32 {
    println!("What final grade do you want to recieve? ");
    let mut ft = String::new();

    loop {
        io::stdin().read_line(&mut ft).expect("Failed to read line");

        let ft: u32 = match ft.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return ft;
    }
}

fn finalHomework() -> u32 {
    println!("What final grade do you want to recieve? ");
    let mut fhw = String::new();

    loop {
        io::stdin()
            .read_line(&mut fhw)
            .expect("Failed to read line");

        let fhw: u32 = match fhw.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return fhw;
    }
}

fn main() {
    let b = -1;

    while b != 4 {
        let b = displayIntro();

        if b == 1 {
            let tsaw = testScoreWeight();
            let prw = projectWeight();
            let hww = homeworkWeight();
            let otw = otherWeight();

            let tsa = testScoreAverage();
            let pr = projectAverage();
            let hw = homeworkAverage();
            let ot = otherAverage();

            let tsaw1 = tsaw / 100;
            let prw1 = prw / 100;
            let hww1 = hww / 100;
            let otw1= otw / 100;

            let tsa2 = tsaw1 * tsa;
            let pr2 = prw1 * pr;
            let hw2 = hww1 * hw;
            let otw2 = otw1 * ot;

            let f = tsa2 + pr2 + hw2 + otw2;

            println!("Your final grade is - {}%", f);
        }
        if b == 2 {
            let tsaw = testScoreWeight();
            let prw = projectWeight();
            let hww = homeworkWeight();
            let otw = otherWeight();
            let pr = projectAverage();
            let hw = homeworkAverage();
            let ot = otherAverage();

            let ftt = finalTest();

            let tsaw1 = tsaw / 100;
            let prw1 = prw / 100;
            let hww1 = hww / 100;
            let otw1= otw / 100;

            let ftt1 = ftt / 100;

            let pr2 = prw1 * pr;
            let hw2 = hww1 * hw;
            let otw2 = otw1 * ot;

            let var1 = ftt1 - pr2 - hw2 - otw2;

            let var2 = (var1 / tsaw1) * 100;

            println!(
                "Your test grade needed to finish with a {}% is - {}%",
                ftt, var2
            );
        }
        if b == 3 {
            let tsaw = testScoreWeight();
            let prw = projectWeight();
            let hww = homeworkWeight();
            let otw = otherWeight();
            let pr = projectAverage();
            let tsa = testScoreAverage();
            let ot = otherAverage();

            let ftt1 = finalHomework();

            let tsaw1 = tsaw / 100;
            let prw1 = prw / 100;
            let hww1 = hww / 100;
            let otw1= otw / 100;

            let ftt1 = ftt1 / 100;

            let tsa2 = tsaw1 * tsa;
            let pr2 = prw1 * pr;
            let otw2 = otw1 * ot;


            let var3 = ftt1 - pr2 - tsa2 - otw2;

            let var4 = (var3 / hww1) * 100;

            println!(
                "Your homework grade needed to finish with a {}% is - {}%",
                ftt1, var4
            );
        }
    }
}
