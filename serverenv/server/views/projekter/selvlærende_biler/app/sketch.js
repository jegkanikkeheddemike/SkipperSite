var canvas;
var generations = [];
var cgen = 0;
var isRunning = true;
var timeInThisGen = 0;
var timePerGen = 45;  //SECONDS
var scaleV = 1;

var cam = {
  x: 100,
  y: 100
};

var b1;
function setup() {
  try {
    resetSim();
    frameRate(300);
    genSize = document.getElementById("getGenSize").value;
    mutationFactor = document.getElementById("getMutationRate").value;
    width = windowWidth - 300;
    height = windowHeight;
    canvas = createCanvas(width, height);
    canvas.position(300, 0);
    canvas.style("z-index", "-1");
    println("sketch Started")
    createBane()
    makeFirstGen();

  } catch (e) {
    println(e);
  }
}

function draw() {
  try {


    moveCam();
    if (isRunning) {
      background(20,150,20);
      push();
      scale(scaleV)
      translate((width / scaleV) / 2 - cam.x - (150 / scaleV), (height / scaleV) / 2 - cam.y);
      timeInThisGen++;

      generations[cgen].gen.forEach(element => {
        element.step();
      });
      if (checkAllDead()) {
        makeFirstGen();
      }
      drawBane();
      baneDelList.forEach((elements) => {
        stroke(150);
        strokeWeight(5);
        elements.draw();
      });


      generations[cgen].gen.forEach(element => {
        element.drawBil();
      });



      pop();
      drawTimer();
      if (timeInThisGen > 60 * timePerGen) {
        goToNextGen();
      }
    } else {
      background(20,150,20);
      push();
      scale(scaleV)
      translate((width / 2) / scaleV - cam.x - (150 / scaleV), (height / 2) / scaleV - cam.y);
      drawBane();
      baneDelList.forEach((elements) => {
        stroke(150);
        strokeWeight(5);
        elements.draw();
      });
      generations[cgen].gen.forEach(element => {
        element.drawBil();
      });


      pop();
      drawTimer();
      drawPaused();
    }
  } catch (e) {
    isRunning = false;
    println(e);
  }
}

function resetSim() {
  generations = []
  baneDelList = []
  kantList = []
  ballList = []
  cgen = 0
  timeInThisGen = 0

}




function getBestCar() { //Den her crasher hvis alle er døde
  let gen = [...generations[cgen].gen]; //LAVER KOPI

  gen.sort((a, b) => {
    return b.score - a.score;
  });

  let best = gen[0];
  let i = 1;
  while (best.isDead) {
    best = gen[i];
    i++;
  }
  return best;
}

function moveCam() {
  let bil = getBestCar();
  let dx = bil.x - cam.x;
  let dy = bil.y - cam.y;
  cam.x += dx / 30;
  cam.y += dy / 30;
}


function keyPressed() {
  if (key == ' ') {
    isRunning = !isRunning;
  }
}


function drawTimer() {
  let procent = timeInThisGen / (60 * timePerGen);

  let x = 20;
  let y = 50;
  let w = 200;
  let h = 40;

  fill(0, 0, 255, 255);
  noStroke();
  rect(x, y, w * procent, h);


  fill(0, 0, 0, 255);
  stroke(0);
  strokeWeight(1);
  textSize(30);
  text("Time left", x, (y / 2) + 10);
  textSize(20);
  strokeWeight(3);
  fill(0, 0, 0, 0);

  rect(x, y, w, h);


}

function drawPaused() {
  fill(0);
  stroke(0);
  strokeWeight(1);
  textSize(40);
  text("Paused", width - 200, 40);
}