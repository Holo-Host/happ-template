
/*
 * Try-o-rama Scenerio Testing
 */
const path = require('path')
const tape = require('tape')

const { Orchestrator, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/try-o-rama')
const spawnConductor = require('./spawn_conductors')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "..", "dist", "happ-example.dna.json")
const dna = Orchestrator.dna(dnaPath, 'example')
const commonConductorConfig = {
    instances: {
	app: dna,
    },
}

const debugLog = false

const orchestratorSimple = new Orchestrator({
    conductors: {
	alice: commonConductorConfig,
	bob: commonConductorConfig,
	carol: commonConductorConfig,
    },
    debugLog,
    executor: tapeExecutor(tape),
    middleware: backwardCompatibilityMiddleware,
})

// Basic non-scenario tests, using just plain 'tape' test harness

const MIN_EXPECTED_SCENARIOS = 1

const registerAllScenarios = () => {
    let numRegistered = 0

    const registerer = orchestrator => {
	const f = (...info) => {
	    numRegistered += 1
	    return orchestrator.registerScenario(...info)
	}
	f.only = (...info) => {
	    numRegistered += 1
	    return orchestrator.registerScenario.only(...info)
	}
	return f
    }

    // Tee up all of the available Scenario tests
    require('./smoke')(registerer(orchestratorSimple))

    return numRegistered
}

// Alice owes fees at/above the fee payment threshold; she initiates payment of these fees.

// Alice sees here .fees reduce to 0 and her .payable has increased by fees amount, then (after fee
// collection is complete), her .balance and .payable reduces by the fee payment amount, and the fee
// payment transaction is complete in her transaction history.

const runScenarioTests = async () => {
  const alice = await spawnConductor('alice', 3000)
  await orchestratorSimple.registerConductor({name: 'alice', url: 'http://0.0.0.0:3000'})
  const bob = await spawnConductor('bob', 4000)
  await orchestratorSimple.registerConductor({name: 'bob', url: 'http://0.0.0.0:4000'})
  const carol = await spawnConductor('carol', 5000)
  await orchestratorSimple.registerConductor({name: 'carol', url: 'http://0.0.0.0:5000'})

  const delay = ms => new Promise(resolve => setTimeout(resolve, ms))
  console.log("Waiting for conductors to settle...")
  await delay(5000)
  console.log("Ok, starting tests!")

  await orchestratorSimple.run()

  alice.kill()
  bob.kill()
  carol.kill()
}

const run = async () => {
    const num = registerAllScenarios()
    
    // Check to see that we haven't accidentally disabled a bunch of scenarios
    if (num < MIN_EXPECTED_SCENARIOS) {
	console.error(`Expected at least ${MIN_EXPECTED_SCENARIOS}, but only ${num} were registered!`)
	process.exit(1)
    } else {
	console.log(`Registered ${num} scenarios (at least ${MIN_EXPECTED_SCENARIOS} were expected)`)
    }

    await runScenarioTests() // Run try-o-rama tests
    process.exit()
}

run()
