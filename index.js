const test = () => {
	
	let obj = {
		"nooe" : "okda"
	}
	
	let arr = ["reddd",'bleeee']
	
	const str = "nice"
	const cb = () => {}
	
	return (
		<div id = "osx" {...obj}>
			<Nav {...obj} />
			<button id={str}>odd {...arr} money</button>
			<input id={str} />
			<listener onclick={()=>{cb}}></listener>
		</div>
	)
}

function Nav({props}) {
	return (
		<nav {...props}></nav>
	)
}

test()