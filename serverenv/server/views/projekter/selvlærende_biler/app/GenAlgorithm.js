var genSize; //Sættes værdi på i html
var mutationFactor; //Sættes værdi på i html

function makeFirstGen() {
    let firstGeneration = [];

    for (let i = 0; i < genSize; i++) {
        firstGeneration.push(new Bil(
            Math.random() / 4,
            Math.random() * 300,
            Math.random() * 300,
            Math.random() * 10,
            Math.random() * 1.5
        ));
        //reactTurn, reactFrontDist, reactSideDist , maxSpeed, brakePower
    }
    generations = [];
    cgen = 0;
    generations.push({
        gen: firstGeneration,
        bestScore: -1,
        avergScore: -1
    });
}

function goToNextGen() {
    timeInThisGen = 0;
    let gen = generations[cgen].gen;
    let alive = [];
    let genBest = -1;
    let genAverg = 0;
    let bestBil;
    gen.forEach(bil => {
        alive.push(bil);    //Sets all to alive. Since dead can breed, and count towards average
        genAverg += bil.score;
    });
    genAverg = genAverg / alive.length;

    let pool = [];
    alive.forEach(bil => {
        if (bil.score > genBest) {
            genBest = bil.score;
            bestBil = bil;
        }
        for (let i = 0; i < bil.score / 100; i++) {
            pool.push(bil);
        }
    });

    let chrildren = [];


    // Make a copy of the best



    while (chrildren.length < genSize - 1) {

        let b1 = pool[Math.floor(Math.random() * pool.length)];
        let b2 = pool[Math.floor(Math.random() * pool.length)];


        let cReactTurn;
        let cReactFrontDist;
        let cReactSideDist;
        let cMaxSpeed;
        let cBrakePower;

        let r;
        let mutation;

        mutation = 1 + Math.random() * (mutationFactor) - mutationFactor / 2;
        r = Math.random();
        if (r > 0.5) {
            cReactTurn = b1.reactTurn * mutation;
        } else {
            cReactTurn = b2.reactTurn * mutation;
        }

        mutation = 1 + Math.random() * (mutationFactor) - mutationFactor / 2;
        r = Math.random();
        if (r > 0.5) {
            cReactFrontDist = b1.reactFrontDist * mutation;
        } else {
            cReactFrontDist = b2.reactFrontDist * mutation;
        }
        mutation = 1 + Math.random() * (mutationFactor) - mutationFactor / 2;
        r = Math.random();
        if (r > 0.5) {
            cReactSideDist = b1.reactSideDist * mutation;
        } else {
            cReactSideDist = b2.reactSideDist * mutation;
        }

        mutation = 1 + Math.random() * (mutationFactor) - mutationFactor / 2;
        r = Math.random();
        if (r > 0.5) {
            cMaxSpeed = b1.maxSpeed * mutation;
        } else {
            cMaxSpeed = b2.maxSpeed * mutation;
        }

        mutation = 1 + Math.random() * (mutationFactor) - mutationFactor / 2;
        r = Math.random();
        if (r > 0.5) {
            cBrakePower = b1.brakePower * mutation;
        } else {
            cBrakePower = b2.brakePower * mutation;
        }

        let child = new Bil(cReactTurn, cReactFrontDist, cReactSideDist, cMaxSpeed, cBrakePower);

        chrildren.push(child);
    }
    let bestChild = new Bil(
        bestBil.reactTurn,
        bestBil.reactFrontDist,
        bestBil.reactSideDist,
        bestBil.maxSpeed,
        bestBil.brakePower
    );

    bestChild.color = bestBil.color;
    bestChild.bestLastGen = true;

    chrildren.push(bestChild);


    generations[cgen].bestScore = genBest;
    generations[cgen].avergScore = genAverg;
    println("__________");
    println("Gen: " + cgen);
    println("Best: " + Math.floor(generations[cgen].bestScore));
    println("Average: " + Math.floor(generations[cgen].avergScore))

    println(
        "ReactTurn: " + (bestChild.reactTurn + "").substr(0, 4) +
        "<br> reactFrontDist: " + (bestChild.reactFrontDist + "").substr(0, 3) +
        "<br> reactSideDist: " + (bestChild.reactSideDist + "").substr(0, 3) +
        "<br> maxSpeed: " + (bestChild.maxSpeed + "").substr(0, 4) +
        "<br> brakePower: " + (bestChild.brakePower + "").substr(0, 4)

    )

    generations.push({
        gen: chrildren,
        bestScore: -1,
        avergScore: -1
    });
    cgen++;
}


function checkAllDead() {
    let returning = true;
    generations[cgen].gen.forEach(bil => {
        if (!bil.isDead) {
            returning = false;
        }
    });
    return returning;
}