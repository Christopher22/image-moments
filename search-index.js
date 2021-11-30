var searchIndex = JSON.parse('{\
"image_moments":{"doc":"Efficient and compile-time checked calculations of image …","t":[3,18,18,18,18,18,18,18,18,18,18,18,18,18,18,3,8,3,18,18,3,8,8,3,8,8,18,18,18,18,18,18,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,10,11,11,11,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,10],"n":["Central","EPSILON","EPSILON","F1_12","F1_12","F1_2","F1_2","F1_20","F1_20","F1_24","F1_24","F1_6","F1_6","F1_60","F1_60","Index","Moments","NormalizedCentral","ONE","ONE","Order","Point","Scalar","Spatial","SupportedIndex","SupportedOrder","THREE","THREE","TWO","TWO","ZERO","ZERO","abs","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","copysign","default","default","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from_iter","get","get","get","get","into","into","into","into","into","mul_add","ne","ne","ne","powi","sqrt","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","x","y"],"q":["image_moments","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["The central moments of an image or contour which are …","The smallest incremental step of this floating-point …","The smallest incremental step of this floating-point …","The value of the scalar matching 1/12.","The value of the scalar matching 1/12.","The value of the scalar matching 1/2.","The value of the scalar matching 1/2.","The value of the scalar matching 1/20.","The value of the scalar matching 1/20.","The value of the scalar matching 1/24.","The value of the scalar matching 1/24.","The value of the scalar matching 1/6.","The value of the scalar matching 1/6.","The value of the scalar matching 1/60.","The value of the scalar matching 1/60.","An index specified at compile time.","A generalization over different moments.","The normalized central moments of an image or contour …","The value of the scalar matching 1.0.","The value of the scalar matching 1.0.","An order of image moments specified at compile time.","A generalization over different possible representations …","The type of floating-point number used to calculate the …","The raw, spatial moments of an image or contour.","An marker trait indicating a compile-time index is valid …","An order currently supported by this crate for calculating …","The value of the scalar matching 3.0.","The value of the scalar matching 3.0.","The value of the scalar matching 2.0.","The value of the scalar matching 2.0.","The value of the scalar matching 0.0.","The value of the scalar matching 0.0.","Computes the absolute value of self","","","","","","","","","","","","","","","","","","","","","Returns a number composed of the magnitude of self and the …","","","","","","","","","","","","","","","","","","","","","Get the moment at a specific position checked at compile …","","","","","","","","","Fused multiply-add. Computes (self * a) + b with only one …","","","","Raises a number to an integer power.","Calculate the square root.","","","","","","","","","","","","","","","","","","","","","The x position of the point casted into the required …","The y position of the point casted into the required …"],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,0,0,0,0,0,0,1,1,1,1,1,1,1,2,3,4,5,6,2,3,4,5,6,2,3,4,5,6,2,3,4,5,6,1,3,5,2,3,4,5,6,2,3,4,5,6,2,2,3,4,4,5,6,6,7,2,4,6,2,3,4,5,6,1,2,4,6,1,1,2,3,4,5,6,2,3,4,5,6,2,3,4,5,6,2,3,4,5,6,8,8],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["central",3]],[[],["index",3]],[[],["normalizedcentral",3]],[[],["order",3]],[[],["spatial",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["index",3]],[[],["order",3]],[[["central",3]],["bool",15]],[[["index",3]],["bool",15]],[[["normalizedcentral",3]],["bool",15]],[[["order",3]],["bool",15]],[[["spatial",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["spatial",3]]],[[]],[[]],[[["central",3]]],[[]],[[]],[[["intoiterator",8]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["central",3]],["bool",15]],[[["normalizedcentral",3]],["bool",15]],[[["spatial",3]],["bool",15]],[[["i32",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]]],"p":[[8,"Scalar"],[3,"Central"],[3,"Index"],[3,"NormalizedCentral"],[3,"Order"],[3,"Spatial"],[8,"Moments"],[8,"Point"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};