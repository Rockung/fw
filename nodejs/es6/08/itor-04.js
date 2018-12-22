const permissionMap = new Map();
permissionMap.set("admin", {read: true, write: true, del: true});
permissionMap.set("student", {read: true, write: false, del: false});
permissionMap.set("faculty", {read: true, write: true, del: false});
permissionMap.set("staff", {read: true, write: false, del: true});

const permissions= permissionMap[Symbol.iterator]();
console.log(permissions.next());
console.log(permissions.next());
console.log(permissions.next());
