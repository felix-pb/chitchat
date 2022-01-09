import Vue from "vue";
import Vuetify from "vuetify/lib/framework";
import { Color } from "@/components/AppColor.vue";
import { Theme } from "@/components/AppTheme.vue";

Vue.use(Vuetify);

// The default theme is dark, and the default color is teal.
document.body.style.backgroundColor = Theme.dark;
export default new Vuetify({
  theme: {
    dark: true,
    themes: {
      dark: {
        primary: Color.teal,
      },
      light: {
        primary: Color.teal,
      },
    },
  },
});
