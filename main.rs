//web rocket
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// make accesible csv crate
extern crate csv;
//to delete accents
extern crate unidecode;
use unidecode::unidecode;
// Import libraries
use std::io;

//for time/date
use chrono::prelude::*;
use chrono::offset::LocalResult;
//postgresql
extern crate postgres;
use postgres::{Client, NoTls, Error};


use rocket::get;
//1 incorrect //0 correct
static mut correct_id: i32=1;
static mut correct_cedula_for_p: i32=1;
static mut correct_name: i32=1;
static mut correct_gender: i32=1;
static mut correct_civil: i32=1;
static mut correct_nac: i32=1;
static mut correct_phone: i32=1;
static mut correct_adrs: i32=1;
static mut correct_mail: i32=1;

//validado field=> 0 validado correct ; 1 validado incorrect
static mut validado:i32=1;
static mut observado:i32=1;


//#[get("/")]
//fn index() -> &'static str {
//    "Hello, world!"
//}

//main function
fn main() -> Result<(), Error> {
    //rocket::ignite().mount("/", routes![index]).launch();
    //client postgres
    let mut client = Client::connect("postgresql://postgres:passwrd@localhost:5432/postgres", NoTls)?;
    //create  table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS PERSONA (
            id              VARCHAR,
            name            VARCHAR,
            gender         VARCHAR,
            civil           VARCHAR,
            nac          VARCHAR,
            phone           VARCHAR,
            adrs            VARCHAR,
            mail            VARCHAR,
            validado        INT,
            observacion     VARCHAR
            )
    ")?;
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).delimiter(b';').from_reader(io::stdin());
    // loop que recorre todas las filas del csv
    for result in rdr.records() {
        let record = result.expect("a CSV record");
	//assing the first row (id) to a variable
	let str_id = &record[0].to_string();
	//check if the id is correct
    unsafe {check_id(str_id.to_string());}
    //print if the id is correct
    //unsafe {println!("{}", &correct_id);}
    
    //check if the name is correct
    let str_gender=&record[2].to_string();
    //check if the name is correct
    unsafe {check_gender(str_gender.to_string());}
    //print if the name is correct
    //unsafe {println!("{}", &correct_gender);}

    //check if the civil is correct
    let str_civil=&record[3].to_string();
    //check if the civil is correct
    unsafe {check_civil(str_civil.to_string());}
    //print if the civil is correct
    //unsafe {println!("{}", &correct_civil);}

    //check if the nac is correct
    let str_nac=&record[4].to_string();
    //check if the nac is correct
    unsafe {check_nac(str_nac.to_string());}
    //print if the nac is correct
    //unsafe {println!("{}", &correct_nac);}

    //check if the phone is correct
    let str_phone=&record[5].to_string();
    //check if the phone is correct
    unsafe {check_phone(str_phone.to_string());}
    //print if the phone is correct
    //unsafe {println!("{}", &correct_phone);}

    //check if the adrs is correct
    let str_adrs=&record[6].to_string();
    //check if the adrs is correct
    unsafe {check_adrs(str_adrs.to_string());}
    //print if the adrs is correct
    //unsafe {println!("{}", &correct_adrs);}

    //check if the mail is correct
    let str_mail=&record[7].to_string();
    //check if the adrs is correct
    unsafe {check_mail(str_mail.to_string());}
    //print if the mail is correct
    //unsafe {println!("{}", &correct_mail);}

    //check if the name is correct
    let str_name=&record[1].to_string();
    //check if the name is correct
    //unsafe {println!("{}", &correct_name);}
    unsafe {let (final_name,final_last_name)=check_name(str_name.to_string());
    //println!("{} {}", final_name,final_last_name);
    let final_complete_name = format!("{} {}", final_name,final_last_name);
    //println!("{}", final_complete_name);
    let check_observado = vec![correct_id,correct_name,correct_gender,correct_civil,correct_nac,correct_phone,correct_adrs,correct_mail];
    let mut fail_observado=vec![];
    let mut descr_observado=vec![];
    let mut string_observado:String;
    let string_correcto:String="correcto".to_string();
    //println!("{:?}", check_observado);
    
    //unsafe{println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} ", correct_id,correct_name,correct_gender,correct_civil,correct_nac,correct_phone,correct_adrs,correct_mail);}
    //all correct
    if correct_id==0 && correct_name==0 && correct_gender==0 && correct_civil==0 && correct_nac==0 && correct_phone==0 && correct_adrs==0 && correct_mail==0{
        validado=0;
        observado=0;
        //correct way to insert data into table
        client.execute(
            "INSERT INTO PERSONA (id,name,gender,civil,nac,phone,adrs,mail,validado,observacion) VALUES ($1, $2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[str_id,&final_complete_name, str_gender,str_civil,str_nac,str_phone,str_adrs,str_mail,&validado,&string_correcto],
        )?;
        //validate just with correct name phone and mail
    }else if correct_name==0 && correct_phone==0 && correct_mail==0{
        for i in 0..8{
            if check_observado[i]==1{
                fail_observado.push(i);
                
            }
        }
        //println!("{:?}",fail_observado);
        for i in 0..fail_observado.len(){
            if fail_observado[i]==0{
                descr_observado.push("id");
                descr_observado.push(" ");
            } else if fail_observado[i]==1{
                descr_observado.push("name");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==2{
                descr_observado.push("gender");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==3{
                descr_observado.push("civil");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==4{
                descr_observado.push("nac");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==5{
                descr_observado.push("phone");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==6{
                descr_observado.push("adrs");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==7{
                descr_observado.push("mail");
                descr_observado.push(" ");
            }
        }
        string_observado=descr_observado.concat();
        //println!("{:?}",string_observado);
        validado=0;
        observado=1;
        client.execute(
            "INSERT INTO PERSONA (id,name,gender,civil,nac,phone,adrs,mail,validado,observacion) VALUES ($1, $2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[str_id,&final_complete_name, str_gender,str_civil,str_nac,str_phone,str_adrs,str_mail,&validado,&string_observado],
        )?;

    }
    else {
        for i in 0..8{
            if check_observado[i]==1{
                fail_observado.push(i);
                
            }
        }
        //println!("{:?}",fail_observado);
        for i in 0..fail_observado.len(){
            if fail_observado[i]==0{
                descr_observado.push("id");
                descr_observado.push(" ");
            } else if fail_observado[i]==1{
                descr_observado.push("name");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==2{
                descr_observado.push("gender");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==3{
                descr_observado.push("civil");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==4{
                descr_observado.push("nac");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==5{
                descr_observado.push("phone");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==6{
                descr_observado.push("adrs");
                descr_observado.push(" ");
            }
            else if fail_observado[i]==7{
                descr_observado.push("mail");
                descr_observado.push(" ");
            }
        }
        string_observado=descr_observado.concat();
        //println!("{:?}",string_observado);
        validado=1;
        observado=1;
        client.execute(
            "INSERT INTO PERSONA (id,name,gender,civil,nac,phone,adrs,mail,validado,observacion) VALUES ($1, $2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[str_id,&final_complete_name, str_gender,str_civil,str_nac,str_phone,str_adrs,str_mail,&validado,&string_observado],
        )?;
    }
    }



    //println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", &record[0],&record[1],&record[2],&record[3],&record[4],&record[5],&record[6],&record[7]);
    }
    Ok(())
}

//check mail
unsafe fn check_mail (mail: String) {
    let mut mail_split_white: Vec<&str> = mail.split_whitespace().collect();
    let len_mail_split_white=mail_split_white.len();
    if len_mail_split_white<2{
        let mail_split: Vec<&str> = mail.split("@").collect();
        let first_part_mail=mail_split[0];
        let second_part_mail=mail_split[1];
        let len_first_part_mail=first_part_mail.len();
        let last_first_part_email=&first_part_mail[(len_first_part_mail-1)..len_first_part_mail];
        let begin_second_part_email=&second_part_mail[0..1];
        if  last_first_part_email != "." && begin_second_part_email !="."{
            let split_second_part_mail: Vec<&str> = second_part_mail.split(".").collect();
            let dominio=split_second_part_mail[0];
            let len_dominio=dominio.len();
            if len_dominio>=2 && len_dominio<=6{
                let chars_first_part_mail: Vec<char> = first_part_mail.chars().collect();
                let chars_are_alpha_mail: Vec<bool> = first_part_mail.chars().map(|c|c.is_alphanumeric()).collect();
                let check_alpha_mail = !chars_are_alpha_mail.contains(&false); 
                if check_alpha_mail{
                    correct_mail=0;
                }else {
                    let mut check_mail_chars=1;
                    let dot:char ='.';
                    let guion:char ='-';
                    let guion_b:char ='_';
                    for i in 0.. chars_first_part_mail.len(){
                        if chars_first_part_mail[i].eq(&dot) || chars_first_part_mail[i].eq(&guion) || chars_first_part_mail[i].eq(&guion_b){
                            check_mail_chars=0;
                            //println!("{}",mail);
                        }
                    }
                    if check_mail_chars==0{
                        correct_mail=0;
                    }else{
                        correct_mail=1;
                    }
                }
            }else{
                correct_mail=1;    
            }

        }else{
            correct_mail=1;
        }
       
    }else{
        correct_mail=1;
    }
    
}

//check adrs
unsafe fn check_adrs (adrs: String) {
    let mut adrs_split: Vec<&str> = adrs.split_whitespace().collect();
    let adrs_length=adrs_split.len();
    if adrs_length>=2{
        correct_adrs=0;
    }else{
        correct_adrs=1;
    }   
}

//check phone
unsafe fn check_phone (phone: String) {
    let chars_are_numeric_phone: Vec<bool> = phone.chars().map(|c|c.is_numeric()).collect();
    let phone_numeric = !chars_are_numeric_phone.contains(&false);
    correct_phone=1;
    if phone_numeric{
        let len_phone=phone.chars().count();
        if len_phone==9 ||len_phone==11 || len_phone==10 || len_phone>=6 && len_phone !=9 && len_phone !=11 && len_phone !=10{
	        match len_phone{
	            9 => check_conv(phone),
                11 => check_conv_pre(phone),
                10 => check_celu(phone),
	            _ => check_another_phone(phone),
	        }


        }
    }
    else{
	correct_phone=1;
    }
}

//check conv phone
unsafe fn check_conv (phone: String) {
    let array_valid_conv =["02","03","04","05","06","07"];
    let first_digits_conv = &phone[0..2];
    let mut check_first_digits_conv=1;
    for n in 0..6{
        if first_digits_conv==array_valid_conv[n]{
            check_first_digits_conv=0;
        }
    }
    if check_first_digits_conv==0{
        correct_phone=0;
    }else{
        correct_phone=1;
    }
    

}

//check conv phone with code
unsafe fn check_conv_pre (phone: String) {
    let first_digits_conv_pre = &phone[0..3];
    let third_digit_conv_pre=&phone[3..4];
    let array_valid_conv_pre =["2","3","4","5","6","7"];
    let mut check_third_digit_conv_pre=1;
    for n in 0..6{
        if third_digit_conv_pre==array_valid_conv_pre[n]{
            check_third_digit_conv_pre=0;
        }
    }
    if first_digits_conv_pre=="593" && check_third_digit_conv_pre==0{
        correct_phone=0;
    }else{
        correct_phone=1;
    } 
    
}

//check celu phone
unsafe fn check_celu (phone: String) {
    let first_digits_celu = &phone[0..2];
    if first_digits_celu=="09"{
        correct_phone=0;
    }else{
        correct_phone=1;
    }
}

//check another phone
unsafe fn check_another_phone (phone: String) {
    let array_valid_another_phone =["54","55","56","57","51"];
    let first_digits_another_phone = &phone[0..2];
    let mut check_digits_another_phone=1;
    for n in 0..5{
        if first_digits_another_phone==array_valid_another_phone[n]{
            check_digits_another_phone=0;
        }
    }
    if check_digits_another_phone==0{
        correct_phone=0;
    }else{
        correct_phone=1;
    }
    
}


//check nac
unsafe fn check_nac (nac: String) {
    //check format
    let nac_split: Vec<&str> = nac.split("-").collect();
    //check if contain 3 elementes yyyy-MM-dd
    if nac_split.len()==3{
        let nac_year_len=nac_split[0].len();
        let nac_month_len=nac_split[1].len();
        let nac_day_len=nac_split[2].len();
        let nac_year:i32=nac_split[0].parse().unwrap();
        let nac_month:i32=nac_split[1].parse().unwrap();
        let nac_day:i32=nac_split[2].parse().unwrap();
        let current_date=Utc::now().date().format("%Y-%m-%d").to_string();
        let current_date_split: Vec<&str> = current_date.split("-").collect();
        let current_year:i32=current_date_split[0].parse().unwrap();
        let current_month:i32=current_date_split[1].parse().unwrap();
        let current_day:i32=current_date_split[2].parse().unwrap();
        let mut check_year=1;
        correct_nac=1;
        if current_year-nac_year>=8 && current_year-nac_year<=95{
            check_year=0;
        } 
        if nac_year_len==4 &&  nac_month_len==2 && nac_day_len==2  {
            if nac_month >=1 && nac_month <=12 && nac_day>=1 && nac_day<=31 && check_year==0{ 
                correct_nac=0;
            }
            else{
                correct_nac=1;
            }       
        }
    }else{
        correct_nac=1;
    }
}

//check civil
unsafe fn check_civil (civil: String) {
    let array_valid_civil =["SOLTERO","CASADO","DIVORCIADO","VIUDO","UNION DE HECHO","NULL"];
    let array_invalid_civil=["UNION LIBRE","SEPARADO"];
    let mut check_valid_civil=1;
    for n in 0..6{
        if civil.to_uppercase()==array_valid_civil[n]{
            check_valid_civil=0;
        }
    }
    if check_valid_civil==0{
        correct_civil=0;
    }else{
        correct_civil=1;
    }
}


//check gender
unsafe fn check_gender (gender: String) {
    let array_valid_gender =["M","F","NULL"];
    let mut check_valid_gender=1;
    for n in 0..3{
        if gender==array_valid_gender[n]{
            check_valid_gender=0;
        }
    }
    if check_valid_gender==0{
        correct_gender=0;
    }else{
        correct_gender=1;
    }

}

//check name
unsafe fn check_name (name: String) -> (String,String) {
    //println!("{}",name);
    correct_name=1;
    let copy_name:String=name.to_string();
    let copy_name1:String=name.to_string();
    if name.contains(char::is_whitespace){
        let mut name_split: Vec<&str> = name.split_whitespace().collect();
        let name_length=name_split.len();
        let mut check_string_name_alphabetic =false; 
        let mut check_string_name_upper = false;
        let new_name=unidecode(name_split[0]);
        let last_name=unidecode(name_split[1]);
        for (i, x) in name_split.iter().enumerate() {
            check_string_name_alphabetic = x.chars().all(|c: char| c.is_alphabetic());
            check_string_name_upper = x.chars().all(|c: char| c.is_uppercase());
        }
        if check_string_name_alphabetic && check_string_name_upper && name_length>=2{
            correct_name=0;
        }else if name_length>=2 {
            //check company name
            let mut chars_are_alphanumeric_cname: Vec<bool> = name.chars().map(|c|c.is_alphanumeric()).collect();
            let string_to_vec: Vec<char> = name.chars().collect();
            let white:char =' ';
            for i in 0..chars_are_alphanumeric_cname.len(){
                if chars_are_alphanumeric_cname[i]==false && string_to_vec[i].eq(&white){
                    chars_are_alphanumeric_cname[i]=true;
                }
            }
            let id_alphanumeric_cname = !chars_are_alphanumeric_cname.contains(&false);
            //println!("{:?}",chars_are_alphanumeric_cname);
            if id_alphanumeric_cname{
                correct_name=0;
            }else{
                let chars_cname: Vec<char> = name.chars().collect();
                let mut check_cname_chars=1;
                let ampersand:char ='&';
                let guion:char ='-';
                let dot:char ='.';
                let left_par:char ='(';
                let right_par:char =')';
                let slash:char ='/';
                let arroba:char ='@';
                let com_smpl:char ='\'';
                //println!("{}",name);
                for i in 0.. chars_cname.len(){
                    if chars_cname[i].eq(&ampersand) || chars_cname[i].eq(&guion) || chars_cname[i].eq(&dot) || chars_cname[i].eq(&left_par) || chars_cname[i].eq(&right_par) || chars_cname[i].eq(&slash) || chars_cname[i].eq(&arroba)  || chars_cname[i].eq(&com_smpl){
                        check_cname_chars=0;
                        //println!("{}",mail);
                    }
                }
                if check_cname_chars==0{
                    correct_name=0;
                }else{
                    correct_name=1;
                }
            }
            
        }
        return (new_name,last_name);
    }
    else {
        correct_name=1;
        return (copy_name,copy_name1);
    }   
}




//check id
unsafe fn check_id (value: String) {
    let len=value.chars().count();
    if len==10 || len==13 || len>=5 && len<=20 && len!=13 && len!=10{
	match len{
	    10 => check_cedula(value),
	    13 => check_RUC(value),
	    _ => check_pasaporte(value),
	}


    }
    else{
	correct_id=1;
    }
}

unsafe fn check_cedula (cedula:String) {
    let chars_are_numeric_id: Vec<bool> = cedula.chars().map(|c|c.is_numeric()).collect();
    let id_numeric = !chars_are_numeric_id.contains(&false);
    //verify that the id only contains digits
    if id_numeric {
        let array_valid_digits =["01","02","03","04","05","06","07","08","09","10","11","12","13","14","15","16","17","18","19","20","21","22","23","24","30","50","80"];
        let n_cedula: usize = cedula.parse().unwrap();
        let first_digits = &cedula[0..2];
        let mut check_valid_digits=1;
        for n in 0..27{
	        //0 if two digits are correct, 1 if are incorrect
	        if first_digits == array_valid_digits[n]{
	            check_valid_digits=0;
	        }
        }
        //verify digit    Coefficients = 2.1.2.1.2.1.2.1.2
        let n1_ced_string= &cedula[0..1];
        let n1_ced: i32 = n1_ced_string.parse().unwrap();
        let n2_ced_string= &cedula[1..2];
        let n2_ced: i32 = n2_ced_string.parse().unwrap();
        let n3_ced_string= &cedula[2..3];
        let n3_ced: i32 = n3_ced_string.parse().unwrap();
        let n4_ced_string= &cedula[3..4];
        let n4_ced: i32 = n4_ced_string.parse().unwrap();
        let n5_ced_string= &cedula[4..5];
        let n5_ced: i32 = n5_ced_string.parse().unwrap();
        let n6_ced_string= &cedula[5..6];
        let n6_ced: i32 = n6_ced_string.parse().unwrap();
        let n7_ced_string= &cedula[6..7];
        let n7_ced: i32 = n7_ced_string.parse().unwrap();
        let n8_ced_string= &cedula[7..8];
        let n8_ced: i32 = n8_ced_string.parse().unwrap();
        let n9_ced_string= &cedula[8..9];
        let n9_ced: i32 = n9_ced_string.parse().unwrap();
        let n10_ced_string= &cedula[9..10];
        let n10_ced: i32 = n10_ced_string.parse().unwrap();
        let mut x1= 2*n1_ced; if x1 >9 {x1=x1-9;}
        let mut x2= 1*n2_ced; if x2 >9 {x2=x2-9;} 
        let mut x3= 2*n3_ced; if x3 >9 {x3=x3-9;}
        let mut x4= 1*n4_ced; if x4 >9 {x4=x4-9;}
        let mut x5= 2*n5_ced; if x5 >9 {x5=x5-9;}
        let mut x6= 1*n6_ced; if x6 >9 {x6=x6-9;}
        let mut x7= 2*n7_ced; if x7 >9 {x7=x7-9;}
        let mut x8= 1*n8_ced; if x8 >9 {x8=x8-9;}
        let mut x9= 2*n9_ced; if x9 >9 {x9=x9-9;}
        let mut x_total=x1+x2+x3+x4+x5+x6+x7+x8+x9;
        let module_x_total= x_total-(x_total % 10);
        let big_module_x_total=module_x_total+10;
        let mut n_verify_digit=big_module_x_total-x_total;
        if n_verify_digit==10 {n_verify_digit=0;}
        //verify digit, if two digits are correct and final consumer=999999999   
        if n_verify_digit==n10_ced && check_valid_digits==0 || n_cedula==9999999999 {correct_id=0; correct_cedula_for_p=0; } else{correct_id=1;correct_cedula_for_p=1;}
        
        
    //cedula que contiene letras
    }else {
        correct_id=1;
        correct_cedula_for_p=1;
    }


    

    
    
}

unsafe fn check_RUC (ruc:String) {
    let chars_are_numeric_ruc: Vec<bool> = ruc.chars().map(|c|c.is_numeric()).collect();
    let ruc_numeric = !chars_are_numeric_ruc.contains(&false);
    let n_ruc: usize = ruc.parse().unwrap();
    let array_valid_digits_ruc =["01","02","03","04","05","06","07","08","09","10","11","12","13","14","15","16","17","18","19","20","21","22","23","24"];
    let first_digits_ruc = &ruc[0..2];
    let mut check_valid_digits_ruc=1;
    if ruc_numeric{
        for n in 0..24{
	        //0 if two digits are correct, 1 if are incorrect
	        if first_digits_ruc == array_valid_digits_ruc[n]{
	            check_valid_digits_ruc=0;
	        }
        }
        //check last 3 digits != 000
        let last_digits_ruc=&ruc[10..13];
        if last_digits_ruc != "000" && (last_digits_ruc == "001" || last_digits_ruc == "002" || last_digits_ruc == "003") {
            //verify digit 

            let third_digit_ruc=&ruc[2..3];
            let n1_ruc_string= &ruc[0..1];
            let n1_ruc: i32 = n1_ruc_string.parse().unwrap();
            let n2_ruc_string= &ruc[1..2];
            let n2_ruc: i32 = n2_ruc_string.parse().unwrap();
            let n3_ruc_string= &ruc[2..3];
            let n3_ruc: i32 = n3_ruc_string.parse().unwrap();
            let n4_ruc_string= &ruc[3..4];
            let n4_ruc: i32 = n4_ruc_string.parse().unwrap();
            let n5_ruc_string= &ruc[4..5];
            let n5_ruc: i32 = n5_ruc_string.parse().unwrap();
            let n6_ruc_string= &ruc[5..6];
            let n6_ruc: i32 = n6_ruc_string.parse().unwrap();
            let n7_ruc_string= &ruc[6..7];
            let n7_ruc: i32 = n7_ruc_string.parse().unwrap();
            let n8_ruc_string= &ruc[7..8];
            let n8_ruc: i32 = n8_ruc_string.parse().unwrap();
            let n9_ruc_string= &ruc[8..9];
            let n9_ruc: i32 = n9_ruc_string.parse().unwrap();
            let n10_ruc_string= &ruc[9..10];
            let n10_ruc: i32 = n10_ruc_string.parse().unwrap();
            if third_digit_ruc=="6" {
                //3digit=6 coeff 32765432
                let mut x1= 3*n1_ruc; 
                let mut x2= 2*n2_ruc;  
                let mut x3= 7*n3_ruc; 
                let mut x4= 6*n4_ruc; 
                let mut x5= 5*n5_ruc; 
                let mut x6= 4*n6_ruc; 
                let mut x7= 3*n7_ruc; 
                let mut x8= 2*n8_ruc; 
                let mut x_total=x1+x2+x3+x4+x5+x6+x7+x8;
                let residual_x_total= (x_total % 11);
                let conf_ruc=11-residual_x_total;
                if conf_ruc==n9_ruc{
                    correct_id=0;
                }else{
                    correct_id=1;
                }
                
            }
            else if third_digit_ruc=="9"{
                //3digit=9 coeff 432765432
                let mut x1= 4*n1_ruc; 
                let mut x2= 3*n2_ruc;  
                let mut x3= 2*n3_ruc; 
                let mut x4= 7*n4_ruc; 
                let mut x5= 6*n5_ruc; 
                let mut x6= 5*n6_ruc; 
                let mut x7= 4*n7_ruc; 
                let mut x8= 3*n8_ruc; 
                let mut x9= 2*n9_ruc;
                let mut x_total=x1+x2+x3+x4+x5+x6+x7+x8+x9;
                let residual_x_total= (x_total % 11);
                let conf_ruc=11-residual_x_total;
                if conf_ruc==n10_ruc{
                    correct_id=0;
                }else{
                    correct_id=1;
                }
            }
            else{
                correct_id=1;
            }
            
        }
        else{
            correct_id=1;
        }

    }else{
        correct_id=1;
    }
    
}

unsafe fn check_pasaporte (pasaporte:String) {
    if pasaporte.chars().all(|x| x.is_alphanumeric()){
        let new_p_string: Vec<&str> = pasaporte.split(|c: char| c.is_numeric()).collect();
        let new_p_n: Vec<&str> = pasaporte.split(|c: char| c.is_alphabetic()).collect();
        let pointer_vacio:&str="";
        let mut p_n_part="";
        let mut p_string_part="";
        for (i, x) in new_p_string.iter().enumerate() {
            if x.ne(&pointer_vacio){
                p_string_part=x;
            }
        }
        for (i, x) in new_p_n.iter().enumerate() {
            if x.ne(&pointer_vacio){
                p_n_part=x;
            }   
        }
        let check_string_p = p_string_part.chars().all(|c: char| c.is_uppercase());
        if check_string_p{
            correct_id=0;
        }else{
            correct_id=1;
        }
        //check if the 10 first digits are ci
        if p_n_part.to_string().chars().count()==10 && correct_id==0{
            check_cedula(p_n_part.to_string());
            if correct_cedula_for_p==0{
                correct_id=1;
            //    println!("EL numero es una cedula no de un pasaporte");
            }else{
                correct_id=0;
            }
        }
        
    }
    else{
        correct_id=1;
    }
}























