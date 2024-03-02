use std::{fmt, io};

fn main() {
  let mut artists = Vec::new();

  let jesus = Artist::new(
    String::from("Кожихов Владислав Дмитриевич"),
    String::from("1997-06-12"),
    26,
    String::from("Киров, Кировская область, Россия"),
    String::from("Рок, рэп (хавно), панк"),
  );

  let lil_peep = Artist::new(
    String::from("Густав Элайджа Ар"),
    String::from("1996-08-1"),
    21,
    String::from("Аллентаун, Пенсильвания, США"),
    String::from("Эмо, рэп (хавно), панк"),
  );

  artists.push(jesus);
  artists.push(lil_peep);

  loop {
    println!("What would you like to do?");
    println!("1. Add a Artist");
    println!("2. Quit");

    let action: u32 = get_user_input().parse().expect("Invalid input");

    match action {
      1 => {
        println!("Enter the name: ");
        let name = get_user_input();

        println!("Enter the date of birth: ");
        let date_of_birth = get_user_input();

        println!("Enter the age: ");
        let age = get_user_input().parse().expect("Invalid input");

        println!("Enter the born: ");
        let born = get_user_input();

        println!("Enter the genre: ");
        let genre = get_user_input();

        let new_artist = Artist::new(name, date_of_birth, age, born, genre);
        artists.push(new_artist);

        println!("1. Continue");
        println!("2. exit ?");

        let action: u16 = get_user_input().parse().expect("Invalid input");

        if action == 1 {
          continue;
        } else if action == 2 {
          break;
        }
      }
      2 => {
        break;
      }
      _ => {
        println!("Please enter a number: ");
        continue;
      }
    }
  }

  for i in artists {
    println!("{}", i)
  }
}

struct Artist {
  name: String,
  date_of_birth: String,
  age: u16,
  born: String,
  genre: String,
}

impl fmt::Display for Artist {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "\nName: {},\nDate of Birth: {},\nAge: {},\nPlace of Birth: {},\nGenre: {}",
      self.name, self.date_of_birth, self.age, self.born, self.genre
    )
  }
}

impl Artist {
  fn new(name: String, date_of_birth: String, age: u16, born: String, genre: String) -> Artist {
    Artist {
      name,
      date_of_birth,
      age,
      born,
      genre,
    }
  }
}

fn get_user_input() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  input.trim().to_string()
}
