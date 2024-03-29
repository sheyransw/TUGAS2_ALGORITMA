use std::io;


struct Tugas {
    id: usize,
    title: String,
    description: String,
    deadline: String,
}

fn input_str(prompt: &str) -> String {
    let mut input: String = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    input.trim().to_string()
  }
  
  fn input_usize(prompt: &str) -> usize {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let id = input.trim().parse().unwrap_or(0);
  
    if input.trim().parse::<usize>().is_err() {
      println!("Masukkan ID tugas yang valid.");
      return 0;
    }
  
    id
  }

fn main() {
    // Mendeklarasikan variabel data tugas
    let mut data_tugas: Vec<Tugas> = vec![];

    // Mendeklarasikan variabel stack dan queue
    let mut stack: Vec<Tugas> = vec![];
   

    // Menampilkan menu utama
    loop {
        // Menampilkan judul menu
        println!("== Menu Utama ==");

        // Menampilkan daftar pilihan
        println!("1. Lihat data");
        println!("2. Tambah data");
        println!("3. Edit data");
        println!("4. Hapus data");
        println!("5. Stack");
        println!("6. Queue");
        println!("7. Keluar");

        // Menunggu input dari pengguna
        let mut pilihan = String::new();
        println!("Pilih menu: ");
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca input");

        // Memproses pilihan pengguna
        let pilihan = pilihan.trim();
        match pilihan {
            // Lihat data
            "1" => {
                // Menampilkan data tugas
                for task in &data_tugas {
                    println!("ID: {}", task.id);
                    println!("Judul: {}", task.title);
                    println!("Deskripsi: {}", task.description);
                    println!("Deadline: {}", task.deadline);
                }
            },
            // Tambah data
            "2" => {
                // Menambah data tugas
                let _ = Tugas {
                    id: input_usize("Masukkan ID tugas: "),
                    title: input_str("Masukkan judul tugas: "),
                    description: input_str("Masukkan deskripsi tugas: "),
                    deadline: input_str("Masukkan deadline tugas: "),
                };
                
    

                // Menambah data tugas ke stack
                let task = Tugas {
                    id: input_usize("Masukkan ID tugas: "),
                    title: input_str("Masukkan judul tugas: "),
                    description: input_str("Masukkan deskripsi tugas: "),
                    deadline: input_str("Masukkan deadline tugas: "),
                  };
                
                  stack.push(task);
                
            }
            //edit tugas
            "3" => {
                // Mengedit data tugas
                let index = input_usize("Masukkan ID tugas yang ingin diedit: ");
            
                // Jika data tugas tidak ditemukan
                if index >= data_tugas.len() {
                    println!("Data tugas tidak ditemukan.");
                    continue;
                }
            
                // Mengambil data tugas dari stack
                let mut task = stack.pop().unwrap();
            
                // Mengedit data tugas
                {
                    // Membuat referensi ke objek `Tugas`
                    let task_ref = &mut task;
            
                    task_ref.title = input_str("Masukkan judul tugas baru: ");
                    task_ref.description = input_str("Masukkan deskripsi tugas baru: ");
                    task_ref.deadline = input_str("Masukkan deadline tugas baru: ");
                }
            
                // Menambah data tugas yang telah diedit ke stack
                stack.push(task);
            },
            // Hapus data
            "4" => {
                // Menghapus data tugas
                let index = input_usize("Masukkan ID tugas yang ingin dihapus: ");

                // Jika data tugas tidak ditemukan
                if index >= data_tugas.len() {
                    println!("Data tugas tidak ditemukan.");
                    continue;
                }

                // Menghapus data tugas
                data_tugas.remove(index);
            },
            // Keluar
            "5" => {
                break;
            },
            // Nilai lain
            _ => {
                println!("Pilihan tidak valid.");
            },
        }
    
    }}
