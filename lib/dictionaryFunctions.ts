const getMaxValue = (map: Map<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  let max = -Infinity

  for (const key of keys) {
    max = Math.max(max, map.get(key).at(whichNumber))
  }

  return max
}

const getMinValue = (map: Map<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  let min = Infinity

  for (const key of keys) {
    min = Math.min(min, map.get(key).at(whichNumber))
  }

  return min
}

const getKeysByValue = (map: Map<string, [number, number, number?]>, keys: string[], value: number, whichNumber: number) => {
  const goodKeys: string[] = []
  
  for (const key of keys) {
    if (map.get(key).at(whichNumber) === value) {
      goodKeys.push(key)
    }
  }

  return goodKeys
}

const getKeysByMaxValue = (map: Map<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  return getKeysByValue(map, keys, getMaxValue(map, keys, whichNumber), whichNumber)
}

const getKeysByMinValue = (map: Map<string, [number, number, number?]>, keys: string[], whichNumber: number) => {
  return getKeysByValue(map, keys, getMinValue(map, keys, whichNumber), whichNumber)
}

export { getKeysByMaxValue, getKeysByMinValue }
