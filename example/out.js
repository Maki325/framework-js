async function Coffee(response, { kind }) {
  const url = `https://api.sampleapis.com/coffee${kind}`;
  const res = await fetch(url);
  const coffees = await res.json();
  response.send(
    `<ul>${(() => { const _FlDSgx5TXsTo = coffees.map((coffee) => `<li>${(() => { const _6MgKgAOBnrPl = coffee.title; if (Array.isArray(_6MgKgAOBnrPl)) { return _6MgKgAOBnrPl.join(''); } else if (typeof _6MgKgAOBnrPl === 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _6MgKgAOBnrPl; } })()}</li>`); if (Array.isArray(_FlDSgx5TXsTo)) { return _FlDSgx5TXsTo.join(''); } else if (typeof _FlDSgx5TXsTo === 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _FlDSgx5TXsTo; } })()}</ul>`
  );
}

export default Coffee;
