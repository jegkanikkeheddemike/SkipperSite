var bilWidth = 25;  //total bredde
var bilLength = 40; //total længde
var bilAcc = 0.1;   //Acceleration
var nextBilID = 0;


class Bil {
    x = baneStartX;
    y = baneStartY;
    speed = 0;
    heading = baneStartHeading; //ER I RADIANER
    frontDist = -1; //1 hvis der er mur tæt på, lav hvis langt væk. 0 hvis uden for range
    leftDist = -1;
    rightDist = -1;
    alreadyTouched = []
    score = 0;
    bestLastGen = false;
    constructor(reactTurn, reactFrontDist, reactSideDist, maxSpeed, brakePower) {
        this.reactTurn = reactTurn;
        this.reactFrontDist = reactFrontDist;
        this.reactSideDist = reactSideDist;
        this.maxSpeed = maxSpeed;
        this.brakePower = brakePower;
        this.isDead = false;
        this.hasWon = false;
        this.color = color(Math.random() * 255, Math.random() * 255, Math.random() * 255)
        this.id = nextBilID;
        nextBilID++;
    }

    step() {
        if (!this.isDead && !this.hasWon) {
            if (this.speed < this.maxSpeed) {
                this.speed += bilAcc;
            }
            if (this.frontDist < this.reactFrontDist) {
                this.speed -= this.brakePower * this.frontDist;
            }
            this.turn(-this.leftDist);
            this.turn(this.rightDist);

            this.x += this.speed * sin(this.heading);
            this.y += this.speed * cos(this.heading);

            this.checkWallCollision();

            this.calcFrontDist();
            this.calcLeftDist();
            this.calcRightDist();
            this.seeScoreB();
            this.checkEnd();
        }
        if(this.hasWon){
            this.speed *= 0.995  
            this.x += this.speed * sin(this.heading);
            this.y += this.speed * cos(this.heading);
            this.calcFrontDist();
            this.calcLeftDist();
            this.calcRightDist();
        }

    }

    drawBil() {
        fill(this.color);
        if (this.isDead) {
            fill(100, 100);
        }else if(this.hasWon){
            fill(212, 175, 55);
            
        }
        stroke(0);
        strokeWeight(1);
        let backX = this.x - bilLength * sin(this.heading);
        let backY = this.y - bilLength * cos(this.heading);
        let frontX = this.x + bilLength * sin(this.heading);
        let frontY = this.y + bilLength * cos(this.heading);
        beginShape();
        vertex(backX + bilWidth * sin(this.heading + PI / 2), backY + bilWidth * cos(this.heading + PI / 2));   //BackLeft
        vertex(backX + bilWidth * sin(this.heading - PI / 2), backY + bilWidth * cos(this.heading - PI / 2));   //BackRight
        vertex(frontX + bilWidth * sin(this.heading - PI / 2), frontY + bilWidth * cos(this.heading - PI / 2));   //FrontRight
        vertex(frontX + bilWidth * sin(this.heading + PI / 2), frontY + bilWidth * cos(this.heading + PI / 2));   //FrontLeft
        vertex(backX + bilWidth * sin(this.heading + PI / 2), backY + bilWidth * cos(this.heading + PI / 2));   //BackLeft
        endShape();
        stroke(150);
        //FRONT LINECHECK
        stroke(this.frontDist * 255, 0, 0);
        if (this.frontDist != 0) {
            line(this.x, this.y, this.x + (1 - this.frontDist) * this.reactFrontDist * sin(this.heading), this.y + (1 - this.frontDist) * this.reactFrontDist * cos(this.heading));
        } else {
            line(this.x, this.y, this.x + this.reactFrontDist * sin(this.heading), this.y + this.reactFrontDist * cos(this.heading));
        }
        //LEFT
        stroke(this.leftDist * 255, 0, 0);
        if (this.leftDist != 0) {
            line(this.x, this.y, this.x + (1 - this.leftDist) * this.reactSideDist * sin(this.heading + 3.14 / 4), this.y + (1 - this.leftDist) * this.reactSideDist * cos(this.heading + 3.14 / 4));
        } else {
            line(this.x, this.y, this.x + this.reactSideDist * sin(this.heading + 3.14 / 4), this.y + this.reactSideDist * cos(this.heading + 3.14 / 4));
        }
        //RIGHT
        stroke(this.rightDist * 255, 0, 0);
        if (this.rightDist != 0) {
            line(this.x, this.y, this.x + (1 - this.rightDist) * this.reactSideDist * sin(this.heading - 3.14 / 4), this.y + (1 - this.rightDist) * this.reactSideDist * cos(this.heading - 3.14 / 4));
        } else {
            line(this.x, this.y, this.x + this.reactSideDist * sin(this.heading - 3.14 / 4), this.y + this.reactSideDist * cos(this.heading - 3.14 / 4));
        }
        stroke(0);
        fill(this.color);
        if (this.isDead) {
            fill(100, 100);
            stroke(0);
        }
        if (!this.bestLastGen) {
            text((this.score+"").substr(0,4), this.x, this.y - 40)
        } else {
            text((this.score+"").substr(0,4) + "\nBest last gen", this.x, this.y - 40)
        }

        fill(0);
    }

    calcFrontDist() {
        let lineDist = Infinity;
        kantList.forEach(element => {
            let colData = lineColPoint(
                this.x,
                this.y,
                this.x + this.reactFrontDist * sin(this.heading),
                this.y + this.reactFrontDist * cos(this.heading),
                element.startX,
                element.startY,
                element.slutX,
                element.slutY
            );
            if (colData.col) {
                if (pointDist(this.x, this.y, colData.x, colData.y) < lineDist) {
                    lineDist = pointDist(this.x, this.y, colData.x, colData.y);
                }
            }
        });
        if (lineDist > this.reactFrontDist || lineDist == -1) {
            this.frontDist = 0;
        } else {
            this.frontDist = (this.reactFrontDist - lineDist) / this.reactFrontDist;
        }
    }
    calcLeftDist() {
        let lineDist = Infinity;
        kantList.forEach(element => {
            let colData = lineColPoint(
                this.x,
                this.y,
                this.x + this.reactSideDist * sin(this.heading + 3.14 / 4),
                this.y + this.reactSideDist * cos(this.heading + 3.14 / 4),
                element.startX,
                element.startY,
                element.slutX,
                element.slutY
            );
            if (colData.col) {
                if (pointDist(this.x, this.y, colData.x, colData.y) < lineDist) {
                    lineDist = pointDist(this.x, this.y, colData.x, colData.y);
                }
            }
        });
        if (lineDist > this.reactSideDist || lineDist == -1) {
            this.leftDist = 0;
        } else {
            this.leftDist = (this.reactSideDist - lineDist) / this.reactSideDist;
        }
    }
    calcRightDist() {
        let lineDist = Infinity;
        kantList.forEach(element => {
            let colData = lineColPoint(
                this.x,
                this.y,
                this.x + this.reactSideDist * sin(this.heading - 3.14 / 4),
                this.y + this.reactSideDist * cos(this.heading - 3.14 / 4),
                element.startX,
                element.startY,
                element.slutX,
                element.slutY
            );
            if (colData.col) {
                if (pointDist(this.x, this.y, colData.x, colData.y) < lineDist) {
                    lineDist = pointDist(this.x, this.y, colData.x, colData.y);
                }
            }
        });
        if (lineDist > this.reactSideDist || lineDist == -1) {
            this.rightDist = 0;
        } else {
            this.rightDist = (this.reactSideDist - lineDist) / this.reactSideDist;
        }
    }
    checkWallCollision() {
        let backX = this.x - bilLength * sin(this.heading);
        let backY = this.y - bilLength * cos(this.heading);
        let frontX = this.x + bilLength * sin(this.heading);
        let frontY = this.y + bilLength * cos(this.heading);
        let backRightX = backX + bilWidth * sin(this.heading + PI / 2);
        let backRightY = backY + bilWidth * cos(this.heading + PI / 2);
        let backLeftX = backX + bilWidth * sin(this.heading - PI / 2);
        let backLeftY = backY + bilWidth * cos(this.heading - PI / 2);
        let frontRightX = frontX + bilWidth * sin(this.heading + PI / 2);
        let frontRightY = frontY + bilWidth * cos(this.heading + PI / 2);
        let frontLeftX = frontX + bilWidth * sin(this.heading - PI / 2);
        let frontLeftY = frontY + bilWidth * cos(this.heading - PI / 2);
        let coll = false;
        kantList.forEach(element => {
            if (lineColPoint(backRightX, backRightY, frontRightX, frontRightY, element.startX, element.startY, element.slutX, element.slutY).col) {
                coll = true;
            }
            if (lineColPoint(backLeftX, backLeftY, frontLeftX, frontLeftY, element.startX, element.startY, element.slutX, element.slutY).col) {
                coll = true;
            }
            if (lineColPoint(backRightX, backRightY, backLeftX, backLeftY, element.startX, element.startY, element.slutX, element.slutY).col) {
                coll = true;
            }
            if (lineColPoint(frontRightX, frontRightY, frontLeftX, frontLeftY, element.startX, element.startY, element.slutX, element.slutY).col) {
                coll = true;
            }
        });
        if (coll) {
            this.isDead = true;
        }
    }
    turn(direc) {
        let spd = this.speed;
        let maxSped = 1.3;
        if (spd > -maxSped && spd < 0) {
            spd = -maxSped;
        }
        if (spd < maxSped && spd > 0) {
            spd = maxSped;
        }
        let turning = (direc / (spd * spd)) * this.reactTurn;
        this.heading += turning;
    }
    seeScoreB() {
        ballList.forEach(ball => {

            if (dist(this.x, this.y, ball.bX, ball.bY) <= PBalls.bRadius + bilWidth) {
                if (!this.alreadyTouched.includes(ball)) {
                    this.score += 50
                    this.alreadyTouched.push(ball)
                }
            }
        });
    }
    checkEnd(){
        if(this.score >= 2000){
            if (dist(this.x,this.y,ending.pX,(ending.pY+ending.sPY)/2) <= bilLength && !this.hasWon){
                this.hasWon = true
                this.score += (100/60)*(60*timePerGen-timeInThisGen)
            }
        }



    }
}

function lineColPoint(x1, y1, x2, y2, x3, y3, x4, y4) {
    // calculate the direction of the lines
    let uA = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let uB = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    // if uA and uB are between 0-1, lines are colliding
    if (uA >= 0 && uA <= 1 && uB >= 0 && uB <= 1) {

        // optionally, draw a circle where the lines meet
        let intersectionX = x1 + (uA * (x2 - x1));
        let intersectionY = y1 + (uA * (y2 - y1));
        return {
            col: true,
            x: intersectionX,
            y: intersectionY
        };
    }
    return {
        col: false,
        x: -1,
        y: -1
    };
}

function pointDist(x1, y1, x2, y2) {
    let d = sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
    return d;
}