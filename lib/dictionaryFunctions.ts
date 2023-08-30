const getMaxValue = (dictionary: Record<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  let max = -Infinity

  for (const key of keys) {
    max = Math.max(max, dictionary[key].at(whichNumber))
  }

  return max
}

const getMinValue = (dictionary: Record<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  let min = Infinity

  for (const key of keys) {
    min = Math.min(min, dictionary[key].at(whichNumber))
  }

  return min
}

const getKeysByValue = (dictionary: Record<string, [number, number, number?]>, keys: string[], value: number, whichNumber: number) => {
  const goodKeys: string[] = []
  
  for (const key of keys) {
    if (dictionary[key].at(whichNumber) === value) {
      goodKeys.push(key)
    }
  }

  return goodKeys
}

const getKeysByMaxValue = (dictionary: Record<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  return getKeysByValue(dictionary, keys, getMaxValue(dictionary, keys, whichNumber), whichNumber)
}

const getKeysByMinValue = (dictionary: Record<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  return getKeysByValue(dictionary, keys, getMinValue(dictionary, keys, whichNumber), whichNumber)
}

export { getKeysByMaxValue, getKeysByMinValue }