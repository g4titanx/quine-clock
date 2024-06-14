use std::time::Duration;
use std::thread::sleep;
use chrono::{Local, Timelike};

const F: [u32; 11] = [
    31599, 19812, 14479, 31207, 23524, 29411, 29679,
    30866, 31727, 31719, 1040
];

const S: &str = r#"char*s="?";x,y,d[8],i,dx;f[]={31599,19812,14479,31207,23524,29411,29679,
30866,31727,31719,1040};char*so,*si;p(ch){i=x/2/(3+2);dx=x/2%(3+2);if(i<8&&(y-5)
/2<5&&dx<3&&(f[d[i]]>>((5-(y-5)/2-1)*3+dx))&1)printf("\033[1;41;30m%c\033[0m",ch
);else printf("%c",ch);if(ch=='\n'){y+=1;x=0;}else x+=1;}gd(){time_t t=time(NULL);struct
 tm*tm=localtime(&t);d[0]=tm->tm_hour/10;d[1]=tm->tm_hour%10;d[2]=10;d[3]=tm->tm_min
/10;d[4]=tm->tm_min%10;d[5]=10;d[6]=tm->tm_sec/10;d[7]=tm->tm_sec%10;}main(){for
(gd();;printf("\n\033[%dA\033[%dD",y+1,x),sleep(1),gd())for(so=s,x=0,y=0;*so;so++
)if(*so==63)for(si=s;*si;si++)switch(*si){case'\n':p('\\');p('n');p('"');p('\n')
;p('"');break;case'"':p('\\');p('\"');break;case'\\':p('\\');p('\\');break;default
:p(*si);}else p(*so);}"#;

fn print_char(ch: char, x: &mut usize, y: &mut usize, d: &[usize], f: &[u32]) {
    let i = *x / 2 / (3 + 2);
    let dx = *x / 2 % (3 + 2);
    let y_offset = (*y as isize - 5) / 2;
    if i < 8 && y_offset < 5 && dx < 3 && (f[d[i]] >> (((5 - y_offset - 1) * 3 + dx as isize) as usize)) & 1 != 0 {
        print!("\x1b[1;41;30m{}\x1b[0m", ch);
    } else {
        print!("{}", ch);
    }
    if ch == '\n' {
        *y += 1;
        *x = 0;
    } else {
        *x += 1;
    }
}

fn get_digits() -> [usize; 8] {
    let now = Local::now();
    let mut d = [0; 8];
    d[0] = (now.hour() / 10) as usize;
    d[1] = (now.hour() % 10) as usize;
    d[2] = 10;
    d[3] = (now.minute() / 10) as usize;
    d[4] = (now.minute() % 10) as usize;
    d[5] = 10;
    d[6] = (now.second() / 10) as usize;
    d[7] = (now.second() % 10) as usize;
    d
}

fn main() {
    loop {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let d = get_digits();

        for ch in S.chars() {
            if ch == '?' {
                for si in S.chars() {
                    match si {
                        '\n' => {
                            print_char('\\', &mut x, &mut y, &d, &F);
                            print_char('n', &mut x, &mut y, &d, &F);
                            print_char('"', &mut x, &mut y, &d, &F);
                            print_char('\n', &mut x, &mut y, &d, &F);
                            print_char('"', &mut x, &mut y, &d, &F);
                        }
                        '"' => {
                            print_char('\\', &mut x, &mut y, &d, &F);
                            print_char('"', &mut x, &mut y, &d, &F);
                        }
                        '\\' => {
                            print_char('\\', &mut x, &mut y, &d, &F);
                            print_char('\\', &mut x, &mut y, &d, &F);
                        }
                        _ => {
                            print_char(si, &mut x, &mut y, &d, &F);
                        }
                    }
                }
            } else {
                print_char(ch, &mut x, &mut y, &d, &F);
            }
        }

        println!("\n\x1b[{}A\x1b[{}D", y + 1, x);
        sleep(Duration::from_secs(1));
    }
}
