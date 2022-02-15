var baneDelList = []
var kantList = []

class BaneDel {
    constructor() {
        baneDelList.push(this)
    }
    step() {
    }
    draw() {
    }
}

var cKantX = baneStartX
var cKantY = baneStartY
class Kant extends BaneDel {
    constructor(startX, startY, slutX, slutY) {
        super()
        this.startX = startX
        this.startY = startY
        this.slutX = slutX + cKantX
        this.slutY = slutY + cKantY
        cKantX = this.slutX
        cKantY = this.slutY
        kantList.push(this)
    }

    step() {


    }
    draw() {
        stroke(0);
        strokeWeight(2)
        line(this.startX, this.startY, this.slutX, this.slutY)
    }


}
var ballList = []
class PBalls extends BaneDel {
    bX = 0
    bY = 0
    static bRadius = 20
    constructor(X, Y) {
        super()
        this.bX = X
        this.bY = Y
        ballList.push(this)
    }
    step() {

    }
    draw() {
        strokeWeight(5);
        stroke(0);
        noFill();
        circle(this.bX, this.bY, PBalls.bRadius)
    }


}

class End extends BaneDel {
    constructor(posX, posY, slutPosX, slutPosY) {
        super()
        this.pX = posX
        this.pY = posY
        this.sPX = slutPosX
        this.sPY = slutPosY
    }
    step() {

    }
    draw() {
        stroke(0);
        strokeWeight(10);
        noFill();
        rectMode(CORNERS)
        rect(this.pX, this.pY, this.sPX, this.sPY)
    }
}

function drawBane() {

    let hLength = Math.floor(kantList.length / 2);
    for (let i = 0; i < hLength; i++) {
        let bd1 = kantList[i];
        let bd2 = kantList[i + hLength]
        fill(150);
        strokeWeight(1);
        stroke(150);
        beginShape();
        vertex(bd1.startX, bd1.startY);
        vertex(bd1.slutX, bd1.slutY);
        vertex(bd2.slutX, bd2.slutY);
        vertex(bd2.startX, bd2.startY);
        
        endShape(CLOSE);
    }
}