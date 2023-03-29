// input stream of moving kinematic bodies - entity and position
// if entity is near a cell bound, create a sensor in the neighbor cell
//      only put sensors on the 3 positive directions of the cell
//      this prevents conflicts, as there is only a sensor across a boundary in one direction
// if the sensor hits something, the sensor becomes the object, and the object is removed from the previous cell
