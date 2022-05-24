# Oxidized Guessing Game
A guessing game built in rust where you compete against a (rather basic) AI to find out who can guess the number faster.

## AI
The AI in this project is simply a random number generator where the bounds it can guess between constantly shrink based on the previous guesses. 

For example, if you or it guesses and gets the response that it's "too high" it will never guess that number or higher again. Same thing in reverse for "too low". While basic, this makes the robot guesser more than just a random number generator and gives it a chance to actually beat the player every once in awhile. 

## License
This project is under the [MIT License.](https://choosealicense.com/licenses/mit/)