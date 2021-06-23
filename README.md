<p align="center">
	<img src="buffalo.svg" title="Buffalo">
</p>

# Buffalo

Buffalo is a binary serialization that gives you ultimate control.

## Example

```rust
#[derive(buffalo::Read, buffalo::Write)]
#[buffalo(size = "dynamic")]
struct AddressBook {
	#[buffalo(id = 0, required)]
	pub contacts: Vec<Contact>,
}

#[derive(buffalo::Read, buffalo::Write)]
#[buffalo(size = "dynamic")]
struct Contact {
	#[buffalo(id = 0, required)]
	pub age: u16,
	#[buffalo(id = 1, required)]
	pub name: String,
	#[buffalo(id = 2, required)]
	pub phone_numbers: Option<Vec<PhoneNumber>>,
}

#[derive(buffalo::Read, buffalo::Write)]
#[buffalo(size = "static", value_size = 8)]
enum PhoneNumber {
	#[allow(unused)]
	#[buffalo(id = 0)]
	Home(String),
	#[allow(unused)]
	#[buffalo(id = 1)]
	Mobile(String),
}

fn address_book_example() {
	let mut writer = buffalo::Writer::new();
	let name = writer.write("John Doe");
	let home = writer.write("1231231234");
	let mobile = writer.write("4564564567");
	let phone_numbers = writer.write(&vec![
		PhoneNumberWriter::Home(home),
		PhoneNumberWriter::Mobile(mobile),
	]);
	let contact = writer.write(&ContactWriter {
		age: 28,
		name,
		phone_numbers: Some(phone_numbers),
	});
	let contacts = writer.write(&vec![contact]);
	let address_book = writer.write(&AddressBookWriter { contacts });
	writer.write(&address_book);
	let bytes = writer.into_bytes();
	let address_book = buffalo::read::<AddressBookReader>(&bytes);
	let contact = address_book.contacts().get(0).unwrap();
	assert_eq!(contact.name(), "John Doe");
	assert_eq!(contact.age(), 28);
	let phone_numbers = contact.phone_numbers();
	let home = phone_numbers.unwrap().get(0).unwrap();
	assert_eq!(home.as_home().unwrap(), "1231231234");
	let mobile = phone_numbers.unwrap().get(1).unwrap();
	assert_eq!(mobile.as_mobile().unwrap(), "4564564567");
}
```
