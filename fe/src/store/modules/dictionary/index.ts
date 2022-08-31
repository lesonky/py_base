import { defineStore } from 'pinia';
import * as optionFuns from '@/api/dictionary';
import { TreeSelectOption } from './types';

export type dictionaryKeys = keyof typeof optionFuns;

export type dictionaryStore = {
  [key in dictionaryKeys]: TreeSelectOption[];
};

const useDictionaryStore = defineStore('dictionary', {
  state: (): Partial<dictionaryStore> => {
    return Object.keys(optionFuns).reduce(
      (a: { [key in dictionaryKeys]?: any }, b) => {
        a[b as dictionaryKeys] = undefined;
        return a;
      },
      {}
    );
  },
  actions: {
    updateOptions(partial: Partial<dictionaryStore>) {
      // @ts-ignore-next-line
      this.$patch(partial);
    },
    async initOption(type: dictionaryKeys, useCache = false) {
      try {
        const fun = (optionFuns as any)[type];
        if (fun) {
          // 如果使用缓存,且存储中有值,则不用获取
          if (useCache && this[type]) {
            return;
          }
          const {
            data: { items },
          } = await fun();
          this.updateOptions({ [type]: items });
        }
      } catch (err) {
        console.error(err);
      }
    },
    async translateOption(type: dictionaryKeys, value: string) {
      try {
        let options = this[type];
        if (!options) {
          await this.initOption(type);
        }
        options = this[type] || [];
        const option = options.find((o: TreeSelectOption) => o.value === value);
        return Promise.resolve(option ? option.label : '未知');
      } catch (err) {
        console.error(err);
        return Promise.resolve('未知');
      }
    },
  },
});

export default useDictionaryStore;
