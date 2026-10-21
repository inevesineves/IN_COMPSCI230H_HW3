// define two structs reflecting a natural dependency in your chosen domain
// aka "Active Session" that depends on "User Account"

// One struct should contain a reference to data in the other struct

// Use a lifetime parameter to maintain the relationship
// and guarantee memory safety between the two structs.

struct PlayerAccount<'a> {
    user_id: &'a str,
    username: &'a str,
    connected: bool,
    input_count: u32,
}

struct GameSession<'b> {
    session_id: &'b str,
    gamemode: u8,
    next_player: &'b PlayerAccount<'b>,
}

fn create_PlayerAccount<'b> (user_id: &'b str, username: &'b str) -> PlayerAccount<'b> {
    PlayerAccount {
        user_id,
        username,
        connected: true,
        input_count: 0,
    }
}

fn create_GameSession<'c> (session_id: &'c str, gamemode: u8, next_player: &'c PlayerAccount) -> GameSession<'c> {
    GameSession {
        session_id,
        gamemode,
        next_player,
    }
}

// In your main function, create instances (objects) of these two structs.
// Ensure the object being referenced has a longer lifetime 
// than the object holding the reference. (Hint: use an inner scope.)
// Then write a function that takes an instance of the struct
// holding the reference and also requires a lifetime parameter.

fn main() {
    let player = create_PlayerAccount("12345", "sillyusername");
    {
        let session = create_GameSession("54321", 1, &player);
        let next = next_player_session(&session);
        print!("The next player is: {next}");
    } // this ensures session is dropped before the player account
}

fn next_player_session<'d> (session: &'d GameSession) -> &'d str {
    session.next_player.username
}
