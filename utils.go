package main

// InSlice checks if a stings is in a slice of strings
func InSlice(haystack []string, needle string) bool {
	for _, i := range haystack {
		if i == needle {
			return true
		}
	}

	return false
}
