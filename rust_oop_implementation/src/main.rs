struct BankAccount{
    account_name: String,
    balance: f64,
}
//now there are no class based methods like instance methods in rust but we can implement methods using impl keyword
//Constrictor implemnattioj below
impl BankAccount{
    fn new(account_name: String, initial_balance: f64) -> Self{
        Self{account_name, balance:initial_balance}
    
    }
    fn read_balance(&self) -> f64 {
        self.balance
    }
    //we cannot move the ownership of String which is in heap space
    fn read_account_name(&self) -> String{
        self.account_name.clone()
    }
    //We can borrow the String via referrence
    fn read_account_name_ref(&self) -> &String{
        &self.account_name
    }

}

fn main() {
    println!("Hello, world!");
    let acc_obj = BankAccount::new("Guru".to_string(), 500.25);
    println!("The balance is : {}",acc_obj.read_balance());
    println!("The account name is {}", acc_obj.read_account_name());
    println!("The account name is {}", acc_obj.read_account_name_ref());
}
