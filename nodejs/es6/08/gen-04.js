function getFlightDurations() {
  setTimeout(() => {
    flightIterator.next({
      Qatar: "39h 0m",
      Emirates: "40h 20m"
    });
  }, 1200);
}

function getFlightPrices() {
  setTimeout(function(){
    flightIterator.next({
      Qatar: "$2010",
      Emirates: "$1904"
    })
  }, 1000);
}

function *getFlights() {
  const allFlights = ["Qatar", "Emirates"];
  const flightDurations = yield getFlightDurations();
  const flightPrices = yield getFlightPrices();

  for (let flight of allFlights) {
    console.log(`New York to Auckland takes ${flightDurations[flight]} in ${flight} airlines for around ${flightPrices[flight]}`);
  }
}

const flightIterator = getFlights();
flightIterator.next();

// If we were not using yield here, flightDurations and flightPrices would 
// simply be assigned as undefined since the function itself would return 
// undefined at the time of assignment. Having a yield statement prevents 
// the value from being assigned until the setTimeout function has executed. 
// This is how generators help us write blocking code when using asynchronous 
// operations, making the code look more synchronous.
