use std::io::{self, Write};

fn main() {
    let mut tickets = Vec::new();
    let mut input = String::new();
    let mut next_id = 1;

    loop {
        println!("\n=== Customer Support System ===");
        println!("1. Create new ticket");
        println!("2. View ticket");
        println!("3. Update ticket status");
        println!("4. List all tickets");
        println!("5. Exit");
        
        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let choice: u32 = input.trim().parse().unwrap_or(0);
        input.clear();

        match choice {
            1 => {
                println!("\n=== Create New Ticket ===");
                print!("Enter ticket title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let title = input.trim().to_string();
                input.clear();

                print!("Enter ticket description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let description = input.trim().to_string();
                input.clear();

                println!("\nSelect priority:");
                println!("1. Low");
                println!("2. Medium");
                println!("3. High");
                println!("4. Urgent");
                print!("Enter priority (1-4): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let priority = match input.trim() {
                    "1" => "Low",
                    "2" => "Medium",
                    "3" => "High",
                    "4" => "Urgent",
                    _ => "Low",
                };
                input.clear();

                let ticket = (
                    next_id,
                    title,
                    description,
                    priority.to_string(),
                    "Open".to_string()
                );
                tickets.push(ticket);
                next_id += 1;
                println!("\nTicket created successfully!");
                println!("Ticket ID: {}", next_id - 1);
            }
            2 => {
                println!("\nEnter ticket ID to view: ");
                print!("Ticket ID: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let id: u32 = input.trim().parse().unwrap_or(0);
                input.clear();

                let mut found = false;
                for ticket in &tickets {
                    if ticket.0 == id {
                        found = true;
                        println!("\nTicket Details:");
                        println!("ID: {}", ticket.0);
                        println!("Title: {}", ticket.1);
                        println!("Description: {}", ticket.2);
                        println!("Priority: {}", ticket.3);
                        println!("Status: {}", ticket.4);
                        break;
                    }
                }
                if !found {
                    println!("\nTicket not found!");
                }
            }
            3 => {
                println!("\nEnter ticket ID to update: ");
                print!("Ticket ID: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let id: u32 = input.trim().parse().unwrap_or(0);
                input.clear();

                println!("\nSelect new status:");
                println!("1. Open");
                println!("2. In Progress");
                println!("3. Resolved");
                println!("4. Closed");
                print!("Enter status number (1-4): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let status = match input.trim() {
                    "1" => "Open",
                    "2" => "In Progress",
                    "3" => "Resolved",
                    "4" => "Closed",
                    _ => "Open",
                };
                input.clear();

                let mut found = false;
                for ticket in &mut tickets {
                    if ticket.0 == id {
                        found = true;
                        ticket.4 = status.to_string();
                        println!("\nStatus updated successfully!");
                        println!("New status: {}", status);
                        break;
                    }
                }
                if !found {
                    println!("\nTicket not found!");
                }
            }
            4 => {
                println!("\n=== All Tickets ===");
                for ticket in &tickets {
                    println!("\nTicket ID: {}", ticket.0);
                    println!("Title: {}", ticket.1);
                    println!("Priority: {}", ticket.3);
                    println!("Status: {}", ticket.4);
                }
            }
            5 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("\nInvalid choice! Please try again."),
        }
    }
}
