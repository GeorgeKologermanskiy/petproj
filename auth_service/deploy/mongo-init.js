//db = db.getSiblingDB("admin");
//db.createUser({
//    user: "auth_service",
//    pwd: "password",
//    roles: [{role: "readWrite", db: "auth_service"}],
//    mechanisms: ["SCRAM-SHA-1"],
//});
db.createCollection('users');
db.users.createIndex({ 'user_id': 1, 'email': 1 },  { 'unique' : true });
db.createCollection('confirmations');
db.confirmations.createIndex({ 'user_id': 1 }, { 'unique': true });

//> db.createUser({user: "auth_service", pwd: "password", roles: [{role: "readWrite", db: "auth_service"}], mechanisms: ["SCRAM-SHA-1"]})
//