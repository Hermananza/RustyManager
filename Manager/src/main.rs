mod funcs;
mod text_funcs;

use std::{env, fs, io, thread};
use std::io::BufRead;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use chrono::DateTime;
use rusqlite::Connection;
use crate::text_funcs::{text_to_bold};
use crate::funcs::{create_user, change_limit, change_pass, change_validity, expired_report_json, expired_report_vec, generate_test, is_port_avaliable, remove_user, user_already_exists, users_report_json, users_report_vec, run_command_and_get_output, get_connections, enable_badvpn_port, disable_badvpn_port, enable_proxy_port, disable_proxy_port, online_report_json, online_report, userdata, speedtest_data, enable_checkuser_port, disable_checkuser_port, journald_status, disable_journald, enable_journald, get_services, enable_openvpn, disable_openvpn, restore_backup, make_backup, enable_sslproxy_port, disable_sslproxy_port};

fn main() {
    let sqlite_conn = Connection::open("/opt/rustymanager/db").unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        main_menu(&sqlite_conn);
    } else{
        match (&args[1]).as_str() {
            "--buat-pengguna" => { // Diterjemahkan dari --create-user
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("nama pengguna kosong");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("kata sandi kosong");
                        return;
                    }
                    _i if 4 >= _i => {
                        println!("jumlah hari kosong");
                        return;
                    }
                    _i if 5 >= _i => {
                        println!("batas koneksi kosong");
                        return;
                    }
                    _ => {}
                }


                let days = &args[4];
                let limit = &args[5];

                match days.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("digit tidak valid ditemukan pada hari");
                        return
                    }
                }
                match limit.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("digit tidak valid ditemukan pada batas");
                        return
                    }
                }

                let string = create_user(&args[2], &args[3], days.parse().unwrap(), limit.parse().unwrap(), false, &sqlite_conn);
                println!("{}", string)

            }
            "--hapus-pengguna" => { // Diterjemahkan dari --remove-user
                if 2 >= args.len() {
                    println!("nama pengguna kosong");
                    return;
                }
                let string = remove_user(&args[2], false, &sqlite_conn);
                println!("{}", string);
            }
            "--buat-akun-percobaan" => { // Diterjemahkan dari --generate-test
                if 2 >= args.len() {
                    println!("jumlah menit kosong");
                    return;
                }

                let days = &args[2];

                match days.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("digit tidak valid ditemukan pada menit");
                        return
                    }
                }

                let string = generate_test(days.parse().unwrap(), &sqlite_conn);
                println!("{}", string);
            }
            "--ubah-batas" => { // Diterjemahkan dari --change-limit
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("nama pengguna kosong");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("batas koneksi kosong");
                        return;
                    }
                    _ => {}
                }

                let limit = &args[3];

                match limit.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("digit tidak valid ditemukan pada batas");
                        return
                    }
                }

                let string = change_limit(&args[2], limit.parse().unwrap(), false, &sqlite_conn);
                println!("{}", string);
            }
            "--ubah-validitas" => { // Diterjemahkan dari --change-validity
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("nama pengguna kosong");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("jumlah hari kosong");
                        return;
                    }
                    _ => {}
                }

                let days = &args[3];

                match days.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("digit tidak valid ditemukan pada hari");
                        return
                    }
                }

                let string = change_validity(&args[2], days.parse().unwrap(), false, &sqlite_conn);
                println!("{}", string);
            }
            "--ubah-sandi" => { // Diterjemahkan dari --change-pass
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("nama pengguna kosong");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("kata sandi kosong");
                        return;
                    }
                    _ => {}
                }


                let string = change_pass(&args[2], &args[3], false, &sqlite_conn);
                println!("{}", string);
            }
            "--data-pengguna" => { // Diterjemahkan dari --userdata
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("nama pengguna kosong");
                        return;
                    }
                    _ => {}
                }

                let string = userdata(&args[2], &sqlite_conn);
                println!("{}", string);
            }

            "--laporan-pengguna" => { // Diterjemahkan dari --users-report
                let string = users_report_json(&sqlite_conn);
                println!("{}", string);
            }

            "--laporan-kedaluwarsa" => { // Diterjemahkan dari --expired-report
                let string = expired_report_json(&sqlite_conn);
                println!("{}", string);
            }

            "--laporan-online" => { // Diterjemahkan dari --online-report
                let string = online_report_json(&sqlite_conn);
                println!("{}", string);
            }
            "--hapus-kedaluwarsa" => { // Diterjemahkan dari --remove-expired
                let expired = expired_report_vec(&sqlite_conn);
                if expired.len() > 0 {
                    for user in expired {
                        remove_user(user.user.as_str(), true, &sqlite_conn);
                        println!("dihapus")
                    }
                } else {
                    println!("tidak ditemukan")
                }
            }
            "--buat-cadangan" => { // Diterjemahkan dari --make-backup
                let make = make_backup(&sqlite_conn);
                println!("{}", make);
            }
            "--pulihkan-cadangan" => { // Diterjemahkan dari --restore-backup
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("jalur kosong");
                        return;
                    }
                    _ => {}
                }
                let backup_path = &args[2];
                if Path::new(&backup_path).exists() {
                    let restore = restore_backup(&sqlite_conn, backup_path.to_string());
                    if restore == "backup restored" {
                        println!("{}", restore);
                    }
                } else {
                    println!("file tidak ditemukan di jalur tersebut");
                }

            }
            "--bantuan" => { // Diterjemahkan dari --help
                let mut text = " -- data bantuan".to_owned();
                text = text + "\n   --buat-pengguna <pengguna> <sandi> <hari> <batas>";
                text = text + "\n   --hapus-pengguna <pengguna>";
                text = text + "\n   --buat-akun-percobaan <waktu dalam menit>";
                text = text + "\n   --ubah-batas <pengguna> <batas>";
                text = text + "\n   --ubah-validitas <pengguna> <validitas dalam hari>";
                text = text + "\n   --ubah-sandi <pengguna> <sandi>";
                text = text + "\n   --laporan-pengguna";
                text = text + "\n   --laporan-kedaluwarsa";
                text = text + "\n   --laporan-online";
                text = text + "\n   --buat-cadangan";
                text = text + "\n   --pulihkan-cadangan <jalur cadangan>";

                println!("{}", text)
            }

            _ => {
                println!("fungsi yang dipilih tidak valid")
            }
        }
    }
}

fn user_dont_exists() {
    println!("pengguna ini tidak ada\n\n> Tekan tombol apa saja untuk kembali ke menu");
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("");
}

fn user_exists() {
    println!("pengguna ini sudah ada\n\n> Tekan tombol apa saja untuk kembali ke menu");
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("");
}

fn get_os_info() -> (String, String) {
    let os_info = fs::read_to_string("/etc/os-release").expect("Gagal membaca /etc/os-release");

    let mut os_name = String::new();
    let mut os_version = String::new();

    for line in os_info.lines() {
        if line.starts_with("ID=") {
            os_name = line.trim_start_matches("ID=").trim_matches('"').to_string();
        } else if line.starts_with("VERSION_ID=") {
            os_version = line.trim_start_matches("VERSION_ID=").trim_matches('"').to_string();
        }
    }

    (os_name, os_version)
}


fn main_menu(sqlite_conn: &Connection) {
    loop {
        
        Command::new("clear").status().unwrap();
        println!("{}", text_to_bold("Menghitung penggunaan CPU dan RAM..."));
        let (os, version) = get_os_info();
        let ssh_online = run_command_and_get_output("ps -e -o user= -o cmd= | grep '[s]shd: ' | grep -v 'sshd: root@' | awk '{user=$1; if (user != \"root\" && user != \"sshd\") print user}' | wc -l");
        let ovpn_online = run_command_and_get_output("sed -n '/Common Name/,/ROUTING TABLE/{/Common Name/d;/ROUTING TABLE/q;s/,.*//p}' /etc/openvpn/openvpn-status.log 2>/dev/null | wc -l || echo 0");
        let online = ssh_online.parse::<usize>().unwrap() + ovpn_online.parse::<usize>().unwrap();

        let created = run_command_and_get_output("awk -F: '$3 >= 1000 { C++ } END { print C+0 }' /etc/passwd");
        let cpu_usage = run_command_and_get_output("vmstat 1 2 | tail -n 1 | awk '{print 100 - $15 \"%\"}'");
        let cpu_cores = run_command_and_get_output("nproc");
        let ram_total = run_command_and_get_output("free -m | awk 'NR==2{print $2 \" MB\"}'");
        let ram_usage = run_command_and_get_output("free -m | awk 'NR==2{printf \"%.2f%%\\n\", $3*100/$2}'");

        Command::new("clear").status().unwrap();
        println!("{}", text_to_bold("================= RustyManager ================="));
        println!("------------------------------------------------");
        println!("| {} {:<16} | {} {:<3} |", text_to_bold("OS:"), os, text_to_bold("Pengguna Dibuat:"), created);
        println!("| {} {:<12} | {} {:<4} |", text_to_bold("Versi:"), version, text_to_bold("Pengguna Online:"), online);
        println!("-----------------------|------------------------");
        println!("| {:<28} | {:<29} |", text_to_bold("CPU:"), text_to_bold("RAM:"));
        println!("|  - {} {:<8} |  - {} {:<11} |", text_to_bold("Core:"), cpu_cores, text_to_bold("Total:"), ram_total);
        println!("|  - {} {:<9} |  - {} {:<10} |", text_to_bold("Digunakan:"), cpu_usage, text_to_bold("Digunakan:"), ram_usage);
        println!("------------------------------------------------");
        let options = vec![
            "Kelola Pengguna",
            "Kelola Koneksi",
            "Alat",
        ];

        for (i, option) in options.iter().enumerate() {
            println!("| {:02} - {:<39} |", i + 1, option);
        }
        println!("| 00 - {:<39} |", "Keluar");
        println!("------------------------------------------------");
        println!("\n --> Pilih salah satu opsi:");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    0 => { break }
                    1 => {
                        users_menu(&sqlite_conn);
                    }
                    2 => {
                        connection_menu(&sqlite_conn);
                    }
                    3 =>{
                        utils_menu(&sqlite_conn)
                    }

                    _ => {}
                }
            }
            Err(err) => {
                println!("{}", err);
                println!("Pilih opsi yang valid")
            }
        }
    }
}
fn users_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|              {}              |", text_to_bold("Kelola Pengguna"));
        println!("------------------------------------------------");
        let options = vec![
            "Buat Pengguna",
            "Hapus Pengguna",
            "Buat Akun Percobaan",
            "Ubah Batas",
            "Ubah Validitas",
            "Ubah Kata Sandi",
            "Laporan Pengguna",
            "Laporan Pengguna Kedaluwarsa",
            "Laporan Pengguna Terhubung",
            "Hapus Pengguna Kedaluwarsa"
        ];

        for (i, option) in options.iter().enumerate() {
            println!("| {:02} - {:<39} |", i + 1, option);
        }
        println!("| 00 - {:<39} |", "Kembali ke menu");
        println!("------------------------------------------------");
        println!("\n --> Pilih salah satu opsi:");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();


        match option.trim().parse() {
            Ok(op) => {
                match op {
                    0 => { break }
                    1 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: buat pengguna");
                        println!("Masukkan nama pengguna: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if user_already_exists(user.as_str()) {
                            user_exists();
                            continue
                        }

                        println!("Masukkan kata sandi: ");
                        let mut pass = String::new();
                        io::stdin().read_line(&mut pass).unwrap();
                        pass = pass.trim().to_string();
                        if pass.is_empty() {
                            continue;
                        }

                        let mut days = String::new();
                        loop {
                            println!("Masukkan masa berlaku dalam hari: ");
                            if !days.is_empty() {
                                days = String::new()
                            }
                            io::stdin().read_line(&mut days).unwrap();
                            days = days.trim().to_string();
                            match days.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("masukkan angka yang valid");
                                }
                            }
                        }

                        let mut limit = String::new();
                        loop {
                            println!("Masukkan batas koneksi: ");
                            if !limit.is_empty() {
                                limit = String::new()
                            }
                            io::stdin().read_line(&mut limit).unwrap();
                            limit = limit.trim().to_string();
                            match limit.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("masukkan angka yang valid");
                                }
                            }
                        }
                        Command::new("clear").status().unwrap();

                        let create = create_user(&*user, &*pass, days.parse().unwrap(), limit.parse().unwrap(), true, &sqlite_conn);
                        match create.as_str() {
                            "created" => {
                                let mut text = ">>> Pengguna berhasil dibuat".to_owned();
                                text = text + "\n - Pengguna: " + &*user;
                                text = text + "\n - Kata Sandi: " + &*pass;
                                text = text + "\n - Hari hingga kedaluwarsa: " + &*days;
                                text = text + "\n - Batas koneksi: " + &*limit;
                                text = text + "\n\n> Tekan tombol apa saja untuk kembali ke menu";
                                println!("{}", text);
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            "error on insert user in db" => {
                                println!("pengguna telah dibuat, tetapi terjadi kesalahan saat menyimpannya di database\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: hapus pengguna");
                        println!("Masukkan nama pengguna: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let remove = remove_user(&*user, true, &sqlite_conn);
                        match remove.as_str() {
                            "removed" => {
                                println!(">>> Pengguna berhasil dihapus\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            "error on remove user at db" => {
                                println!("pengguna telah dihapus, tetapi terjadi kesalahan saat mencoba menghapusnya dari database\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            _ => {}
                        }
                    }
                    3 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: buat akun percobaan");
                        let mut minutes = String::new();
                        loop {
                            println!("Masukkan waktu kedaluwarsa dalam menit: ");
                            if !minutes.is_empty() {
                                minutes = String::new()
                            }
                            io::stdin().read_line(&mut minutes).unwrap();
                            minutes = minutes.trim().to_string();
                            match minutes.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("masukkan angka yang valid");
                                }
                            }
                        }

                        let gen = generate_test(minutes.parse().unwrap(), &sqlite_conn);
                        match gen.as_str() {
                            "error on insert user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("akun percobaan telah dibuat, tetapi terjadi kesalahan saat menyimpannya di database\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            _ => {
                                if gen.contains("user: ") {
                                    Command::new("clear").status().unwrap();
                                    let mut text = ">>> Akun percobaan berhasil dibuat".to_owned();
                                    let user = gen.split("user: ").collect::<Vec<&str>>()[1].split(" |").collect::<Vec<&str>>()[0];
                                    text = text + "\n - Pengguna: " + &*user;
                                    text = text + "\n - Kata Sandi: " + &*user;
                                    text = text + "\n - Kedaluwarsa dalam: " + &*minutes + " menit";
                                    text = text + "\n - Batas koneksi: 1";
                                    text = text + "\n\n> Tekan tombol apa saja untuk kembali ke menu";
                                    println!("{}", text);
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }

                            }
                        }
                    }
                    4 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: ubah batas");
                        println!("Masukkan nama pengguna: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let mut limit = String::new();
                        loop {
                            println!("Masukkan batas baru: ");
                            if !limit.is_empty() {
                                limit = String::new()
                            }
                            io::stdin().read_line(&mut limit).unwrap();
                            limit = limit.trim().to_string();
                            match limit.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("masukkan angka yang valid");
                                }
                            }
                        }

                        let change = change_limit(&*user, limit.parse().unwrap(), false, &sqlite_conn);
                        match change.as_str() {
                            "error on update user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("terjadi kesalahan, silakan coba lagi\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            _ => {
                                if change.contains("changed") {
                                    Command::new("clear").status().unwrap();
                                    println!(">>> Batas berhasil diubah\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }

                            }
                        }


                    }
                    5 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: ubah validitas");
                        println!("Masukkan nama pengguna: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let mut days = String::new();
                        loop {
                            println!("Masukkan validitas baru dalam hari: ");
                            if !days.is_empty() {
                                days = String::new()
                            }
                            io::stdin().read_line(&mut days).unwrap();
                            days = days.trim().to_string();
                            match days.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("masukkan angka yang valid");
                                }
                            }
                        }

                        let change = change_validity(&*user, days.parse().unwrap(), false, &sqlite_conn);
                        match change.as_str() {
                            "error on update user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("validitas telah diubah, tetapi terjadi kesalahan saat mencoba memperbaruinya di database\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            _ => {
                                if change.contains("changed") {
                                    Command::new("clear").status().unwrap();
                                    println!(">>> Validitas berhasil diubah\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }
                            }
                        }
                    }
                    6 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: ubah kata sandi");
                        println!("Masukkan nama pengguna: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let mut pass = String::new();
                        println!("Masukkan kata sandi baru: ");
                        io::stdin().read_line(&mut pass).unwrap();
                        pass = pass.trim().to_string();


                        let change = change_pass(&*user, &*pass, false, &sqlite_conn);
                        match change.as_str() {
                            "error on update user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("kata sandi telah diubah, tetapi terjadi kesalahan saat mencoba memperbaruinya di database\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            _ => {
                                if change.contains("changed") {
                                    Command::new("clear").status().unwrap();
                                    println!(">>> Kata sandi berhasil diubah\n\n> Tekan tombol apa saja untuk kembali ke menu");
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }
                            }
                        }
                    }
                    7 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: laporan pengguna");
                        let users = users_report_vec(&sqlite_conn);
                        for user in users {
                            println!("Pengguna: {} | Sandi: {} | Batas: {} | Kedaluwarsa pada: {}", user.user, user.pass, user.limit, DateTime::parse_from_str(&user.expiry, "%Y-%m-%d %H:%M:%S%.3f %z").unwrap().format("%Y-%m-%d"));
                        }
                        println!("\n> Tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    8 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: laporan pengguna kedaluwarsa");
                        let expired = expired_report_vec(&sqlite_conn);
                        for user in expired {
                            println!("Pengguna: {} | Sandi: {} | Batas: {} | Kedaluwarsa pada: {}", user.user, user.pass, user.limit, DateTime::parse_from_str(&user.expiry, "%Y-%m-%d %H:%M:%S%.3f %z").unwrap().format("%Y-%m-%d"));
                        }
                        println!("\n> Tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    9 => {
                        let stdin = io::stdin();
                        let handle = thread::spawn(move || {
                            let _ = stdin.lock().lines().next();
                        });

                        loop {
                            Command::new("clear").status().unwrap();
                            println!("Memantau pengguna yang terhubung melalui SSH");
                            println!("------------------------------------------");
                            println!("Pengguna          | Koneksi/Batas");
                            println!("--------------------------");

                            let users = online_report(&sqlite_conn);
                            let mut total_connections: usize = 0;
                            for user in users {
                                total_connections += user.connected.parse::<usize>().unwrap();
                                println!("{:<18} | {}/{}", user.user, user.connected, user.limit);
                            }
                            if total_connections != 0 {
                                println!("--------------------------");
                            }
                            println!("Total koneksi: {}", total_connections);
                            println!("\n> Tekan tombol apa saja untuk kembali ke menu");

                            if handle.is_finished() {
                                break;
                            }
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                    10 => {
                        Command::new("clear").status().unwrap();
                        println!("--> fungsi dipilih: hapus pengguna kedaluwarsa");
                        let expired = expired_report_vec(&sqlite_conn);
                        if expired.len() > 0 {
                            for user in expired {
                                remove_user(user.user.as_str(), true, &sqlite_conn);
                                println!("pengguna: {} dihapus", user.user);
                            }
                        } else {
                            println!("tidak ada pengguna kedaluwarsa yang ditemukan")
                        }
                        println!("\n> Tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    _ => {}
                }
            }
            Err(err) => {
                println!("{}", err);
                println!("Pilih opsi yang valid")
            }
        }
    }
}

fn connection_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|              {}              |", text_to_bold("Kelola Koneksi"));
        println!("------------------------------------------------");
        println!("| 1 - {:<40} |", "Port Aktif");
        println!("| 2 - {:<40} |", "RustyProxy (ws/wss/socks)");
        println!("| 3 - {:<40} |", "RustyProxySSL (direct/ws/wss)");
        println!("| 4 - {:<40} |", "Badvpn");
        println!("| 5 - {:<40} |", "OpenVpn");
        println!("| 0 - {:<40} |", "Kembali ke menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();

        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        services_menu()
                    }
                    2 => {
                        proxy_menu(&sqlite_conn)
                    }
                    3 => {
                        sslproxy_menu(&sqlite_conn)
                    }
                    4 => {
                        badvpn_menu(&sqlite_conn)
                    }
                    5 => {
                        openvpn_menu(&sqlite_conn)
                    }
                    0 => {
                        break
                    }
                    _ => {
                        Command::new("clear").status().unwrap();
                        println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                }
            }
            Err(_) => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }


    }
}

fn utils_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|                  {}                 |", text_to_bold("Alat"));
        println!("------------------------------------------------");
        println!("| {:<45}|", "1 - Checkuser Multi-Aplikasi");
        println!("| {:<45}|", "2 - Tes Kecepatan");
        println!("| {:<45}|", "3 - Pantau Sumber Daya");
        println!("| {:<45}|", "4 - Kelola Journald");
        println!("| {:<45}|", "5 - Buat Cadangan");
        println!("| {:<45}|", "6 - Pulihkan Cadangan");
        println!("| {:<45}|", "7 - Ubah Kata Sandi Root");
        println!("| {:<45}|", "8 - Mulai Ulang Server");
        println!("| {:<45}|", "0 - Kembali ke menu");
        println!("------------------------------------------------");
        println!();
        let mut option = String::new();
        println!(" --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        checkuser_menu(&sqlite_conn);
                    }
                    2 => {
                        Command::new("clear").status().unwrap();
                        println!("tes sedang berjalan, tindakan ini mungkin memakan waktu...");
                        let speedtest = speedtest_data();
                        let download_bits = speedtest.download.bytes as f64 * 8.0;
                        let upload_bits = speedtest.upload.bytes as f64 * 8.0;

                        let download_mb = download_bits / 1_000_000.0;
                        let upload_mb = upload_bits / 1_000_000.0;

                        let download_seconds = speedtest.download.elapsed as f64 / 1000.0;
                        let upload_seconds = speedtest.upload.elapsed as f64 / 1000.0;

                        let download_mbps = download_mb / download_seconds;
                        let upload_mbps = upload_mb / upload_seconds;

                        Command::new("clear").status().unwrap();

                        println!("------------------------------------------------");
                        println!("|              {}             |", text_to_bold("Tes Kecepatan"));
                        println!("------------------------------------------------");
                        println!("| Jaringan: {:<38} |", speedtest.interface.name);
                        println!("| IP: {:<40} |", speedtest.interface.internal_ip);
                        println!("| Unduh: {:<34} |", format!("{:.2}mbps", download_mbps));
                        println!("| Unggah:   {:<34} |", format!("{:.2}mbps", upload_mbps));
                        println!("| Ping:     {:<32}   |", format!("{:.2}ms", speedtest.ping.latency));
                        println!("------------------------------------------------");

                        println!("\n> tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    3 => {
                        Command::new("clear").status().unwrap();
                        println!("{}", text_to_bold("> peringatan: untuk keluar dari monitor, tekan F10"));
                        println!("> tekan tombol apa saja untuk melanjutkan");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                        Command::new("htop").status().unwrap();
                    }
                    4 => {
                        journald_menu();
                    }
                    5 => {
                        Command::new("clear").status().unwrap();
                        println!("{}", text_to_bold("membuat cadangan..."));
                        let make = make_backup(&sqlite_conn);
                        if make == "backup done in /root/backup.json" {
                            println!("cadangan berhasil dibuat, disimpan di: /root/backup.json")
                        }
                        println!("> tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    6 => {
                        Command::new("clear").status().unwrap();
                        println!("Masukkan jalur file:");
                        let mut backup_path = String::new();
                        io::stdin().read_line(&mut backup_path).expect("");
                        backup_path = backup_path.trim().to_string();

                        if Path::new(&backup_path).exists() {
                            println!("file ditemukan, memulihkan cadangan...\n");
                            let restore = restore_backup(&sqlite_conn, backup_path.to_string());
                            if restore == "backup restored" {
                                println!("\ncadangan berhasil dipulihkan");
                            }
                        } else {
                            println!("\nfile tidak ditemukan di jalur yang dimasukkan");
                        }
                        println!("> tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    7 => {
                        Command::new("clear").status().unwrap();
                        loop {
                            println!("Masukkan kata sandi root baru:");
                            let mut new_pass = String::new();
                            io::stdin().read_line(&mut new_pass).expect("");
                            new_pass = new_pass.trim().to_string();
                            if new_pass.len() >= 4 {
                                run_command_and_get_output(format!("(echo {}; echo {}) | passwd root", new_pass, new_pass).as_str());
                                println!("kata sandi diubah");
                                break;
                            }
                        }
                        println!("> tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    8 => {
                        Command::new("reboot").status().unwrap();
                    }

                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn proxy_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();

        println!("------------------------------------------------");
        println!("|                  {}                 |", text_to_bold("RUSTY PROXY"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let proxy_ports = conn.proxy.ports.unwrap_or_default();
        if proxy_ports.is_empty() {
            println!("| Port: {:<34}|", "tidak ada");
        } else {
            let active_ports = proxy_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Port: {:<34}|", active_ports);
        }

        println!("------------------------------------------------");
        println!("| {:<45}|", "1 - Buka Port");
        println!("| {:<45}|", "2 - Tutup Port");
        println!("| {:<45}|", "0 - Kembali ke menu");
        println!("------------------------------------------------");
        println!();
        let mut option = String::new();
        println!(" --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("port ini sudah digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }
                        println!("Masukkan status koneksi (jangan isi apa pun untuk default): ");
                        let mut status = String::new();
                        io::stdin().read_line(&mut status).unwrap();
                        status = status.trim().to_string();

                        enable_proxy_port(port, status);
                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil diaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("port ini tidak digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }

                        disable_proxy_port(port);
                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil dinonaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}

fn sslproxy_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();

        println!("------------------------------------------------");
        println!("|                  {}              |", text_to_bold("RUSTY PROXY SSL"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let sslproxy_ports = conn.sslproxy.ports.unwrap_or_default();
        if sslproxy_ports.is_empty() {
            println!("| Port: {:<34}|", "tidak ada");
        } else {
            let active_ports = sslproxy_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Port: {:<34}|", active_ports);
        }

        println!("------------------------------------------------");
        println!("| {:<45}|", "1 - Buka Port");
        println!("| {:<45}|", "2 - Tutup Port");
        println!("| {:<45}|", "0 - Kembali ke menu");
        println!("------------------------------------------------");
        println!();
        let mut option = String::new();
        println!(" --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("port ini sudah digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }
                        enable_sslproxy_port(port);
                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil diaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("port ini tidak digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }

                        disable_sslproxy_port(port);
                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil dinonaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn badvpn_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|                    {}                    |", text_to_bold("BADVPN"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let badvpn_ports = conn.badvpn.ports.unwrap_or_default();
        if badvpn_ports.is_empty() {
            println!("| Port: {:<34}|", "tidak ada");
        } else {
            let active_ports = badvpn_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Port: {:<34}|", active_ports);
        }

        println!("| 1 - {:<40} |", "Buka Port");
        println!("| 2 - {:<40} |", "Tutup Port");
        println!("| 0 - {:<40} |", "Kembali ke menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("port ini sudah digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }

                        enable_badvpn_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil diaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("port ini tidak digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }

                        disable_badvpn_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil dinonaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn checkuser_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|                   {}                  |", text_to_bold("CHECKUSER"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let checkuser_ports = conn.checkuser.ports.unwrap_or_default();
        if checkuser_ports.is_empty() {
            println!("| Port: {:<34}|", "tidak ada");
        } else {
            let active_ports = checkuser_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Port: {:<34}|", active_ports);
        }
        println!("| 1 - {:<40} |", "Buka Port");
        println!("| 2 - {:<40} |", "Tutup Port");
        println!("| 0 - {:<40} |", "Kembali ke menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("port ini sudah digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }

                        enable_checkuser_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil diaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukkan port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("port ini tidak digunakan, masukkan yang lain:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("masukkan port yang valid");
                                }
                            }

                        }

                        disable_checkuser_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Port berhasil dinonaktifkan, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}

fn openvpn_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();

        println!("------------------------------------------------");
        println!("|                    {}                   |", text_to_bold("OPENVPN"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let openvpn_port = conn.openvpn.port.unwrap_or_default();
        if openvpn_port.is_empty() {
            println!("| Port: {:<38}|", "tidak ada");
            println!("| 1 - {:<40} |", "Aktifkan OpenVPN");
        } else {
            println!("| Port: {:<38}|", openvpn_port);
            println!("| 1 - {:<40} |", "Nonaktifkan OpenVPN");
        }
        println!("| 0 - {:<40} |", "Kembali ke menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        if openvpn_port.is_empty() {
                            let mut port = String::new();
                            loop {
                                println!("Masukkan port: ");
                                if !port.is_empty() {
                                    port = String::new();
                                };
                                io::stdin().read_line(&mut port).unwrap();
                                port = port.trim().to_string();
                                match port.parse::<usize>() {
                                    Ok(port) => {
                                        if !is_port_avaliable(port).unwrap() {
                                            println!("port ini sudah digunakan, masukkan yang lain:")
                                        } else {
                                            break
                                        }
                                    }
                                    Err(..) => {
                                        println!("masukkan port yang valid");
                                    }
                                }

                            }

                            let mut mode = String::new();
                            loop {
                                println!("Masukkan mode (tcp atau udp): ");
                                if !mode.is_empty() {
                                    mode = String::new();
                                };
                                io::stdin().read_line(&mut mode).unwrap();
                                mode = mode.trim().to_string();
                                println!("mode dipilih: {}", mode);
                                if mode == "tcp" || mode == "udp" {
                                    break
                                }
                            }

                            enable_openvpn(port, mode);

                            Command::new("clear").status().unwrap();
                            println!("\n> OpenVPN berhasil diaktifkan");
                            println!("\n> Sertifikat disimpan di: /root/client.ovpn, tekan tombol apa saja untuk kembali ke menu");
                            let mut return_string = String::new();
                            io::stdin().read_line(&mut return_string).expect("");

                        } else {
                            disable_openvpn();
                            Command::new("clear").status().unwrap();
                            println!("\n> OpenVPN berhasil dinonaktifkan, tekan tombol apa saja untuk kembali ke menu");
                            let mut return_string = String::new();
                            io::stdin().read_line(&mut return_string).expect("");
                        }

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}

fn journald_menu() {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|               {}              |", text_to_bold("Kelola Journald"));
        println!("------------------------------------------------");
        let status = journald_status();
        if status {
            println!("| Status: {:<37}|", "aktif");
            println!("------------------------------------------------");
            println!("| 1 - {:<40} |", "Nonaktifkan");
        } else {
            println!("| Status: {:<37}|", "nonaktif");
            println!("------------------------------------------------");
            println!("| 1 - {:<40} |", "Aktifkan");
        }
        println!("| 0 - {:<40} |", "Kembali ke menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih salah satu opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        if status {
                            disable_journald()
                        } else {
                            enable_journald()
                        }
                        Command::new("clear").status().unwrap();
                        println!("\n> Berhasil, tekan tombol apa saja untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opsi tidak valid, tekan tombol apa saja untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn services_menu() {
    Command::new("clear").status().unwrap();

    println!("------------------------------------------------");
    println!("|                 {}                |", text_to_bold("Port Aktif"));
    println!("------------------------------------------------");
    let services = get_services();
    for service in services {
        println!("| - {:<43}|", format!("{}: {}", service.name, service.ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ")));
    }
    println!("------------------------------------------------");
    println!();
    println!("> Tekan tombol apa saja untuk kembali ke menu");
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("");
}

