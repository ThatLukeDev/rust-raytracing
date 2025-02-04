searchState.loadedDescShard("lib", 0, "A library consisting of raytracing utility functions.\nA camera, consisting of an origin and a rotation matrix.\nA color, in RGB.\nA matrix, of any size, and utility functions.\nShorthand for creating a matrix.\nA ray in 3 dimensions, and utility functions.\nA trait implemented for all scene objects, taking a ray …\nA scene, containing a list of objects, and configurations.\nA sphere, with the Raytrace trait.\nA vector in 3 dimension, and utility functions.\nA camera object, with a postion and a rotation matrix.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new camera instance from a position and a …\nThe position of the camera in 3D space.\nThe direction the camera is facing as a ray.\nThe rotation of the camera as a 3D Matrix.\nRotates a point in camera space to world space.\nA matrix.\nThe error type for a mismatch of sizes between matrices.\nAdds each element in a matrix to the corresponding element.\nReturns the mxnth element, where m and n start at 1.\nClones a matrix, including contents.\nReturns the matrix of cofactors.\nContents of the matrix.\nReturns the determinant of aa matrix.\nScales each element in a matrix by 1 divided by the factor.\nChecks for equality between 2 matrices.\nDisplys the height and width of a matrix, in the format …\nReturns the argument unchanged.\nConverts a Vec3 as a rotation vector to a 3x3 rotation …\nReturns the argument unchanged.\nGetter for height.\nHeight, m, of the matrix.\nCreates a new mxm matrix of the identity matrix.\nReturns a reference to the mxnth element, where m and n …\nReturns a mutable reference to the mxnth element, where m …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nInverses a matrix.\nShorthand for creating a matrix.\nReturns the minor of a matrix.\nReturns the matrix of minors.\nScales each element in a matrix by a factor.\nMatrix multiplication.\nReturns a mutable reference to the mxnth element, where m …\nCreates a new mxn matrix of 0.\nSubtracts each element in a matrix from the corresponding …\nTransposes a matrix.\nGetter for width.\nWidth, n, of the matrix.\nA 3D ray object, that has an origin, and direction.\nThe position of a ray at a distance along its path.\nThe direction of the ray as a unit vector.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nA safe ray creation function, which transforms the …\nThe start position of the ray.\nThe trait for all objects within the raytracer.\nGives the distance at which a ray intersects the object.\nGives the position of intersection between a ray and an …\nReflects, refracts, or otherwise transforms the ray in …\nSphere.\nReturns the argument unchanged.\nGives the distance along a ray that a sphere lies.\nCalls <code>U::from(self)</code>.\nDefault constructor.\nGives the normal to a point on the sphere.\nReflects a ray along the normal.\nThe error type for a mismatch of sizes between the object …\nA point, or vector, or whatever you want that has 3 …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the magnitude of a Vec3.\nReturns the magnitude of a Vec3, squared.\nDefault constructor.\nTakes the vector, and returns a new vector where each part …\nTurns a Vec to a Vec3.\nTurns a Matrix to a Vec3.\nReturns the Vec3 with the same direction, but a magnitude …\nThe x amount.\nThe y amount.\nThe z amount.")