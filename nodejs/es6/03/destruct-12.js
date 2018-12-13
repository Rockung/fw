let student = {
    name: "Tony",
    courses: {
        english: {
            id: 1,
            score: 7 
        },
        math: { id: 2,
            score: 9 
        }
    },
    scoreRange: [0, 10]
};
let {
    courses: { english },
    scoreRange: [ minScore ]
} = student;
console.log(english.id);
console.log(english.score);
console.log(minScore);
// 1 // 7 // 0
