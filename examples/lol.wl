fn fibs(n) {
	if n < 1 then
		0
	else if n < 2 then
		1
	else
		fibs(n-1) + fibs(n-2)
}