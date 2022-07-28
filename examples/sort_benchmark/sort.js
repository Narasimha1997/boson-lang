

const array = new Array(1000000)

console.time('hr_timer')
for(var i = 0; i < 1000000; i++) {
    array[i] = Math.random()
}

array.sort()
console.timeEnd('hr_timer')