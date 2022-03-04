import { Direction, rng_test } from '../../engine/pkg/engine';
import State, { Modes } from './state';
import QRCode from 'qrcode';

let state;
let name;
let board;
let score;
let rank;
let newGame;
let gameOver;
let leaderboardList;
let qrCanvas;
let shareButton;
let touchDownX = null;
let touchDownY = null;

// see https://cupola.gettysburg.edu/cgi/viewcontent.cgi?article=1025&context=csfac
const HIGHEST_POSSIBLE_TILE_VALUE = 131072;

function resetAnimation(elem) {
  elem.style.animation = 'none';
  elem.offsetHeight;
  elem.style.animation = null;
}

function syncBoard(game, board) {

  // Remove hidden tiles
  for (const child of [...board.children]) {
    if (child.classList.contains("hidden")) {
      board.removeChild(child);
    } else {
      child.classList.remove("new");
    }
  }

  const children = [...board.children];

  for (let r = 0; r < 4; r++) {
    for (let c = 0; c < 4; c++) {

      const tile = game.get_tile(r, c);
      if (tile != undefined) {
        const id = `tile-${tile.id}`;
        let div = undefined;
        // Check for existing tile
        for (let i = 0; i < children.length; i++) {
          if (children[i].id == id) {
            div = children[i];
            children.splice(i, 1);
            break;
          }
        }

        if (div == undefined) {
          // Existing tile not found - create new
          div = document.createElement("div");
          div.classList.add("tile");
          div.classList.add("new");
          div.id = id;
          board.appendChild(div);
        }

        const left = `${2 + c * 24.5}%`;
        const top = `${2 + r * 24.5}%`;

        if (tile.merged_with != undefined) {
          const merged_with_id = `tile-${tile.merged_with}`;
          div.classList.add("merged");
          resetAnimation(div);
          for (let i = 0; i < children.length; i++) {
            if (children[i].id == merged_with_id) {
              let toHide = children[i];
              children.splice(i, 1);
              toHide.classList.add("hidden");
              toHide.style.left = left;
              toHide.style.top = top;
              break;
            }
          }
        }

        // Set text and position of tile
        div.innerHTML = `${tile.value}`;
        div.style.left = left;
        div.style.top = top;
        tile.free();
      }

    }
  }

}

function showGameChanger(e) {
  e.stopPropagation();
  state.set_mode(Modes.SWITCH);

  const gameList = document.getElementById("game-list");
  for (const child of [...gameList.children]) {
    if (!child.classList.contains("new-game-slot")) {
      gameList.removeChild(child);
    }
  }
  state.get_all_games().then((games) => {
    for (const game of games) {
      const newBoard = document.createElement("div");
      newBoard.classList.add("game-slot");
      syncBoard(game.game, newBoard);
      newBoard.addEventListener("click", (e) => { e.stopPropagation(); loadGame(game.id); });
      gameList.appendChild(newBoard);
      game.game.free();
    }
    updateUI();
  });
}

function handleBodyClick() {
  if (state.get_mode() != Modes.PLAY) {
    state.set_mode(Modes.PLAY);
    updateUI();
  }
}

function showLeaderboard(e) {
  e.stopPropagation();
  state.set_mode(Modes.LEADERBOARD);
  updateUI();
}

function handleShareCopy(e) {
  e.stopPropagation();

  const baseUrl = window.location.origin;
  const currentGameId = state.get_current_game().get_id();
  const url = `${baseUrl}?game_id=${currentGameId}`;
  navigator.clipboard.writeText(url);
}

function handleShareButton(e) {
  e.stopPropagation();
  const baseUrl = window.location.origin;
  const currentGameId = state.get_current_game().get_id();
  const url = `${baseUrl}?game_id=${currentGameId}`;
  if (navigator.share) {
    navigator.share({
      title: 'twenty48',
      text: "Challenge me on twenty48!",
      url: url
    })
      .then(() => console.log('Successful share'))
      .catch(error => console.log('Error sharing:', error));
  } else {
    state.set_mode(Modes.SHARE);
    QRCode.toCanvas(qrCanvas, url, { width: qrCanvas.offsetWidth }, function (error) {
      if (error) {
        console.error(error);
      } else {
        updateUI();
      }
    })
  }
}

function updateName() {
  state.get_player_name().then((value) => {
    name.classList.add("shown");
    name.value = value;
  });
}

function updateLeaderboard() {
  const leaderboard = state.get_leaderboard();
  let r = 0;
  for (let i = 0; i < leaderboard.length; i++) {
    const entry = leaderboard[i];
    if (entry.requestingPlayer) {
      r = i + 1;
      break;
    }
  }
  if (r > 0) {
    rank.innerHTML = `${r} of ${leaderboard.length}`;

    while (leaderboardList.firstChild) {
      leaderboardList.removeChild(leaderboardList.lastChild);
    }

    for (const entry of leaderboard) {
      const row = document.createElement("div");
      row.classList.add("leaderboard-slot");
      const name = document.createElement("span");
      name.classList.add("leaderboard-name");
      name.innerText = entry.displayName;
      const score = document.createElement("span");
      score.classList.add("leaderboard-score");
      score.innerText = `${entry.score}`;
      row.appendChild(name);
      row.appendChild(score);
      leaderboardList.appendChild(row);
    }

  } else {
    rank.innerHTML = '?';
  }
}

function updateScore(game, score) {
  const newScore = `${game.get_score()}`;
  if (score.innerHTML != newScore) {
    score.classList.add("pop");
    score.innerHTML = newScore;
    resetAnimation(score);
  }
}

function updateOverlays(game, gameOver) {

  const mode = state.get_mode();
  const switchGameContainer = document.getElementById("switch-game-container");
  const leaderboardContainer = document.getElementById("leaderboard-container");
  const shareContainer = document.getElementById("share-container");
  switch (mode) {
    case Modes.PLAY:
      board.classList.remove("blurred");
      switchGameContainer.classList.remove("shown");
      leaderboardContainer.classList.remove("shown");
      shareContainer.classList.remove("shown");
      break;
    case Modes.SWITCH:
      board.classList.add("blurred");
      switchGameContainer.classList.add("shown");
      leaderboardContainer.classList.remove("shown");
      shareContainer.classList.remove("shown");
      break;
    case Modes.LEADERBOARD:
      board.classList.add("blurred");
      switchGameContainer.classList.remove("shown");
      leaderboardContainer.classList.add("shown");
      shareContainer.classList.remove("shown");
      break;
    case Modes.SHARE:
      board.classList.add("blurred");
      switchGameContainer.classList.remove("shown");
      leaderboardContainer.classList.remove("shown");
      shareContainer.classList.add("shown");
      break;
  }

  if (game.get_game_over()) {
    gameOver.classList.add("shown");
    board.classList.add("blurred");
  } else {
    gameOver.classList.remove("shown");
  }
}

function makeMove(direction) {
  if (state.get_mode() == Modes.PLAY) {
    state.make_move(direction).then((newGame) => {
      if (newGame != undefined) {
        updateUI();
      }
    });
  }
}

function handleNameChange(e) {
  e.stopPropagation();
  state.set_player_name(e.target.value);
}

function handleKeyDown(e) {
  e.stopPropagation();
  let direction = undefined;
  switch (e.keyCode) {
    case 37:
      // left arrow
      direction = Direction.Left;
      break;
    case 38:
      // up arrow
      direction = Direction.Up;
      break;
    case 39:
      // right arrow
      direction = Direction.Right;
      break;
    case 40:
      // down arrow
      direction = Direction.Down;
      break;
  }

  if (direction != undefined) {
    makeMove(direction);
  }
}

function handleBoardClick(e) {
  e.preventDefault();
  e.stopPropagation();
  const bounds = board.getBoundingClientRect();
  const x = e.clientX - bounds.left;
  const y = e.clientY - bounds.top;
  const dx = board.offsetWidth / 2 - x;
  const dy = y - board.offsetHeight / 2;
  const angle = (Math.atan2(dy, dx) + (5 / 4) * Math.PI) % (2 * Math.PI);
  let direction;
  if (angle < Math.PI / 2) {
    direction = Direction.Right;
  } else if (angle < Math.PI) {
    direction = Direction.Up;
  } else if (angle < 3 * Math.PI / 2) {
    direction = Direction.Left;
  } else {
    direction = Direction.Down;
  }
  makeMove(direction);
}

function updateUI() {
  updateName();
  const game = state.get_current_game();
  syncBoard(game, board);
  updateScore(game, score);
  updateOverlays(game, gameOver);
}

function initiateGame() {
  state.set_mode(Modes.PLAY);
  for (const child of [...board.children]) {
    if (child.classList.contains("tile")) {
      board.removeChild(child);
    }
  }
  updateUI();
}

function loadGame(id) {
  state.load_game(id).then(initiateGame);
}

function startNewGame(e) {
  e.stopPropagation();
  state.new_game().then(
    initiateGame
  );
}

async function load() {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const gameId = urlParams.get('game_id');
  state = new State();
  if (gameId != null) {
    await state.get_player();
    await state.storeStartupGameId(gameId);
    window.location.replace(window.location.origin);
  } else {
    await state.initialize();
  }
}

function touchStart(e) {
  touchDownX = e.touches[0].clientX;
  touchDownY = e.touches[0].clientY;
};

function touchMove(e) {
  if (touchDownX === null) {
    return;
  }

  if (touchDownY === null) {
    return;
  }

  const dX = touchDownX - e.touches[0].clientX;
  const dY = touchDownY - e.touches[0].clientY;

  if (Math.sqrt(dX * dX + dY * dY) > board.offsetWidth / 4) {

    let direction = undefined;

    if (Math.abs(dX) > Math.abs(dY)) {
      if (dX > 0) {
        direction = Direction.Left;
      } else {
        direction = Direction.Right;
      }
    } else {
      if (dY > 0) {
        direction = Direction.Up;
      } else {
        direction = Direction.Down;
      }
    }

    makeMove(direction);

    touchDownX = null;
    touchDownY = null;
  }

  e.preventDefault();
};

function stopPropagation(e) {
  e.stopPropagation();
}

load().then(() => {

  if (!rng_test(BigInt("18446744073709551615"))) {
    console.log("Random number generator not consitent with binary build!");
  }

  name = document.getElementById("name");
  board = document.getElementById("mainBoard");
  score = document.getElementById("score");
  rank = document.getElementById("rank");
  newGame = document.getElementById("new-game");
  gameOver = document.getElementById("game-over");
  shareButton = document.getElementById("share");
  const shareCopy = document.getElementById("share-copy");
  leaderboardList = document.getElementById("leaderboard-list");
  qrCanvas = document.getElementById("qr-canvas");

  name.addEventListener("input", handleNameChange);

  board.addEventListener("click", handleBoardClick);
  document.addEventListener("keydown", handleKeyDown);

  board.addEventListener("touchstart", touchStart, false);
  board.addEventListener("touchmove", touchMove, false);

  newGame.addEventListener("click", showGameChanger);

  rank.addEventListener("click", showLeaderboard);

  shareButton.addEventListener("click", handleShareButton);

  shareCopy.addEventListener("click", handleShareCopy);

  document.body.addEventListener("click", handleBodyClick);

  const newGameSlot = document.getElementById("new-game-slot");
  newGameSlot.addEventListener("click", startNewGame);

  state.add_player_observer(updateName);

  state.add_leaderboard_observer(updateLeaderboard);

  state.add_game_observer(initiateGame);

  updateUI();

});
