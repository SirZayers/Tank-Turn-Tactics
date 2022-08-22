# Tank Turn Tactics

Inspiration:
https://www.youtube.com/watch?v=aOYbR-Q_4Hs&t=617s
https://www.youtube.com/watch?v=t9WMNuyjm4w

### Rules

- Every player gets a tank randomly placed on the board
- 3 lives each
- Action points can be used for:
  - Shooting another player
  - Moving the tank
  - Improving the tank's range (default: 2)
- When shot a player loses one hitpoint
- Last player standing wins
- Every player gets one action point per day at a random time during the day
- Action points can be donated to another player in range
- Dead players become part of the jury, once per day the jury can vote on which players gets an extra action point (3 votes minimum)
- Actions get logged
- Action points can be saved up for later

### Implementation

Save gamestate in two JSON files (switch between them so in case one gets corrupted during writing, there still is a backup recording the game state before the last move)

Use TCP connection with commands to interact with game:

- Players receive token as identification (from gamemaster until better system is implemented)
- _board_ returns state of gameboard and players (action points, HP, position, range)
- _players_ returns info on players (dead/alive, nickname, contact information, previous game stats?)
- _tank_ (command) controls tank, where commands are _move (up, down, left, right)_, _shoot (player)_, _upgrade_, _donate (player)_
- _log_ brings up logbook
- _vote_ (player name) allows a jury member to cast their vote on which player should receive an extra point

### Future feature wishlist

- Eventually save player accounts seperately from games to track player statistics over multiple games
- Optionally allow non-players to view player stats and games
- Allow players to create games
- Private/invite-only games
- Run multiple games on the same instance
- Allow automated account creation
- In game chats and chat groups to make sharing contact information unneccessary, allow complete tracking of player conversation for fun and research (Anonimizing players also removes potential for damaging real life relationships)
- GUI client for players that don't want to program their own
- Bot-only games
