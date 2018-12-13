class ReversedString extends String {
    print() {
        return this.split('').reverse().join('');
    } 
}
const str = new ReversedString("Awesome");
console.log(str.print());
// emosewA