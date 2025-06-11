import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
interface State {
  classInfoList: ClassInfo[]
  classInfo: ClassInfo | null
}

export const useClassStore = defineStore('class', {
  state: (): State => {
    return {
      classInfoList: [],
      classInfo: null,
    }
  },
  actions: {
    loadClasses() {
      /* Beispieldaten  */
      if (this.classInfoList.length < 1) {
        this.classInfoList.push({ name: 'Steve', id: 1 })
        this.classInfoList.push({ name: 'Garett', id: 2 })
        this.classInfoList.push({ name: 'Natalie', id: 3 })
        this.classInfoList.push({ name: 'Henry', id: 4 })
        this.classInfoList.push({ name: 'Dawn', id: 5 })
      }
    },
    setActiveClass(id: number) {
      this.classInfoList.map((item) => {
        if (item.id == id) {
          this.classInfo = item
        }
      })
    },
  },
})

interface ClassInfo {
  name: string
  id: number
}
